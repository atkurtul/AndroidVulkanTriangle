#![allow(dead_code, unused_variables)]
pub mod vk;
pub use vk::*;
pub mod native_window;
pub use native_window::*;
pub mod input_queue;
pub use input_queue::*;
pub mod event;
pub use event::*;
pub mod callbacks;
pub use callbacks::*;
pub mod log;
pub use log::*;

fn block_until_window(msg: &mpsc::Receiver<MainThreadMsg>) -> (InputQueue, NativeWindow) {
    let mut window : Option<NativeWindow> = None;
    let mut queue : Option<InputQueue> = None;
    loop {
        match msg.recv() {
            Ok(MainThreadMsg::NativeWindowCreated(win)) => window = Some(win),
            Ok(MainThreadMsg::InputQueueCreated(q)) => queue = Some(q),
            Ok(ss) => alog!("Got a msg {:?}", ss.as_str()),
            _ => (),
        }
        if window.is_some() && queue.is_some() {
            return (queue.unwrap(), window.unwrap())
        }
    }
}

fn rust_main(msg: mpsc::Receiver<MainThreadMsg>) {
    a_log("inside rust main".to_owned());
    let instance = create_instance(
        &InstanceCreateInfo::new()
            .enabled_layer_name(&"VK_LAYER_KHRONOS_validation\0".as_ptr())
            .enabled_extension_names(&[
                "VK_KHR_surface\0".as_ptr(),
                "VK_KHR_android_surface\0".as_ptr(),
            ]),
    )
    .unwrap();

    let pdev = instance.enumerate_physical_devices().unwrap()[0];
    let dev = pdev
        .create(
            &DeviceCreateInfo::new()
                .enabled_extension_name(&"VK_KHR_swapchain\0".as_ptr())
                .enabled_layer_name(&"VK_LAYER_KHRONOS_validation\0".as_ptr())
                .queue_create_info(&DeviceQueueCreateInfo::new().queue_priority(&1f32)),
        )
        .unwrap();
    let rqueue = dev.get_queue(0,0);
    a_log("Created device".to_owned());
    let (queue, window) = block_until_window(&msg);
    alog!("got queue {:?}", queue);
    a_log("Got window".to_owned());
    let extent = Extent2D(window.width(), window.height());

    let surface = instance
        .create_android_surface_khr(&AndroidSurfaceCreateInfoKHR::new().window(window.as_void()))
        .unwrap();
    let swapchain = dev
        .create_swapchain_khr(
            &SwapchainCreateInfoKHR::new()
                .surface(surface)
                .image_extent(extent)
                .clipped(1)
                .composite_alpha(CompositeAlphaFlagsKHR::INHERIT_KHR)
                .pre_transform(SurfaceTransformFlagsKHR::IDENTITY_KHR)
                .image_array_layers(1)
                .image_format(Format::R8G8B8A8_SRGB)
                .image_usage(ImageUsageFlags::COLOR_ATTACHMENT)
                .min_image_count(3),
        )
        .unwrap();
    alog!("{:?} {:?}", "Created swapchain", swapchain);

    let renderpass = dev
        .create_render_pass(
            &RenderPassCreateInfo::new()
                .attachment(
                    &AttachmentDescription::new()
                        .format(Format::R8G8B8A8_SRGB)
                        .samples(SampleCountFlags::e1)
                        .load_op(AttachmentLoadOp::CLEAR)
                        .store_op(AttachmentStoreOp::STORE)
                        .initial_layout(ImageLayout::UNDEFINED)
                        .final_layout(ImageLayout::PRESENT_SRC_KHR),
                )
                .subpasse(
                    &SubpassDescription::new()
                        .pipeline_bind_point(PipelineBindPoint::GRAPHICS)
                        .color_attachment(
                            &AttachmentReference::new()
                                .attachment(0)
                                .layout(ImageLayout::COLOR_ATTACHMENT_OPTIMAL),
                        ),
                ),
        )
        .unwrap();
    let images = dev.get_swapchain_images_khr(swapchain).unwrap();
    let views = images
        .iter()
        .map(|&img| {
            dev.create_image_view(
                &ImageViewCreateInfo::new()
                    .format(Format::R8G8B8A8_SRGB)
                    .view_type(ImageViewType::e2D)
                    .image(img)
                    .subresource_range(
                        ImageSubresourceRange::new()
                            .aspect_mask(ImageAspectFlags::COLOR)
                            .level_count(1)
                            .layer_count(1),
                    ),
            )
            .unwrap()
        })
        .collect::<Vec<_>>();
    let framebuffers = views
        .iter()
        .map(|view| {
            dev.create_framebuffer(
                &FramebufferCreateInfo::new()
                    .attachment(&view)
                    .width(extent.0)
                    .height(extent.1)
                    .layers(1)
                    .render_pass(renderpass),
            )
            .unwrap()
        })
        .collect::<Vec<_>>();
    let command_pool = dev
        .create_command_pool(&CommandPoolCreateInfo::new().flags(
            CommandPoolCreateFlags::TRANSIENT |
            CommandPoolCreateFlags::RESET_COMMAND_BUFFER
        ))
        .unwrap();
    let cmd = dev
        .allocate_command_buffers(
            &CommandBufferAllocateInfo::new()
                .command_buffer_count(images.len() as _)
                .level(CommandBufferLevel::PRIMARY)
                .command_pool(command_pool),
        )
        .unwrap();
    let fence = (0..images.len())
        .map(|_| {
            dev.create_fence(&FenceCreateInfo::new().flags(FenceCreateFlags::SIGNALED))
                .unwrap()
        })
        .collect::<Vec<_>>();
    let acq_sp = (0..images.len())
        .map(|_| dev.create_semaphore(&SemaphoreCreateInfo::new()).unwrap())
        .collect::<Vec<_>>();
    let present_sp = (0..images.len())
        .map(|_| dev.create_semaphore(&SemaphoreCreateInfo::new()).unwrap())
        .collect::<Vec<_>>();
    let layout = dev.create_pipeline_layout(&PipelineLayoutCreateInfo::new()).unwrap();
    let vsmodule = dev.create_shader_module(&ShaderModuleCreateInfo::new().code_size(vertex_shader().len()).code(vertex_shader().as_ptr() as _)).unwrap();
    let psmodule = dev.create_shader_module(&ShaderModuleCreateInfo::new().code_size(fragment_shader().len()).code(fragment_shader().as_ptr() as _)).unwrap();
    alog!("Created modules {:?} {:?}", vsmodule.0, psmodule.0);
    let mut pipeline = Pipeline(0);
    dev.create_graphics_pipelines(<_>::default(), &[
    GraphicsPipelineCreateInfo::new()
        .layout(layout)
        .render_pass(renderpass)
        .vertex_input_state(&PipelineVertexInputStateCreateInfo::new())
        .depth_stencil_state(&PipelineDepthStencilStateCreateInfo::new())
        .rasterization_state(
            &PipelineRasterizationStateCreateInfo::new()
                .polygon_mode(PolygonMode::FILL)
                .line_width(1f32),
        )
        .multisample_state(
            &PipelineMultisampleStateCreateInfo::new().rasterization_samples(SampleCountFlags::e1),
        )
        .color_blend_state(&PipelineColorBlendStateCreateInfo::new().attachment(
            &PipelineColorBlendAttachmentState::new().color_write_mask(
                ColorComponentFlags::R
                    | ColorComponentFlags::G
                    | ColorComponentFlags::B
                    | ColorComponentFlags::A,
            ),
        ))
        .viewport_state(
            &PipelineViewportStateCreateInfo::new()
                .viewport(
                    &Viewport::new()
                        .width(extent.0 as _)
                        .height(extent.1 as _)
                        .max_depth(1f32),
                )
                .scissor(&Rect2D::new().extent(extent)),
        )
        .input_assembly_state(
            &PipelineInputAssemblyStateCreateInfo::new().topology(PrimitiveTopology::TRIANGLE_LIST),
        ).stages(&[
	PipelineShaderStageCreateInfo::new()
		.name("main\0".as_ptr())
		.stage(ShaderStageFlags::VERTEX)
		.module(vsmodule),	
	PipelineShaderStageCreateInfo::new()
		.name("main\0".as_ptr())
		.stage(ShaderStageFlags::FRAGMENT)
		.module(psmodule)])], &mut pipeline);

    let mut cc : i32 = -1;
    loop {
        match msg.try_recv() {
            Ok(MainThreadMsg::Destroy()) =>  {
                alog!("Exiting");
                return
            }
            _ => ()
        }
        match queue.get_event() {
            Some(event) => {
                alog!("Got event of type {:?}", event.typ3()) ;
                queue.finish_event(event,1);
            }
            _ => ()
        }
        cc = (cc + 1) % images.len() as i32;
        let idx : usize = cc as _;
        dev.acquire_next_image_khr(swapchain, u64::MAX, acq_sp[idx], Fence(0));
        dev.wait_for_fences(&[fence[idx]], 1, u64::MAX);
        dev.reset_fences(&[fence[idx]]);
        cmd[idx].reset(CommandBufferResetFlags::RELEASE_RESOURCES);
        cmd[idx].begin(&CommandBufferBeginInfo::new()
            .flags(CommandBufferUsageFlags::ONE_TIME_SUBMIT));
        cmd[idx].begin_render_pass(&RenderPassBeginInfo::new()
            .render_pass(renderpass)
            .framebuffer(framebuffers[idx])
            .render_area(Rect2D::new().extent(extent))
            .clear_value(&ClearValue::new()
                        .color(ClearColorValue::new()
                                .float_32([0.4, 0.1, 0.6, 1.0]))), 
                                SubpassContents::INLINE);
        cmd[idx].bind_pipeline(PipelineBindPoint::GRAPHICS, pipeline).draw(3, 1, 0, 0);
        cmd[idx].end_render_pass().end();
        rqueue.submit(&[SubmitInfo::new()
            .command_buffer(&cmd[idx])
            .wait_semaphore(&acq_sp[idx])
            .signal_semaphore(&present_sp[idx])
            .wait_dst_stage_mask(&PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)], 
            fence[idx]);
        rqueue.present_khr(&PresentInfoKHR::new()
            .swapchain(&swapchain)
            .wait_semaphore(&present_sp[idx])
            .image_indices(&(idx as u32)));
    }
}



const fn fragment_shader() -> [u8; 608] {
[0x03, 0x02, 0x23, 0x07, 0x00, 0x00, 0x01, 0x00, 0x0a, 0x00, 0x0d, 0x00, 0x13, 0x00, 0x00, 0x00,
0x00, 0x00, 0x00, 0x00, 0x11, 0x00, 0x02, 0x00, 0x01, 0x00, 0x00, 0x00, 0x0b, 0x00, 0x06, 0x00,
0x01, 0x00, 0x00, 0x00, 0x47, 0x4c, 0x53, 0x4c, 0x2e, 0x73, 0x74, 0x64, 0x2e, 0x34, 0x35, 0x30,
0x00, 0x00, 0x00, 0x00, 0x0e, 0x00, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00,
0x0f, 0x00, 0x07, 0x00, 0x04, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x6d, 0x61, 0x69, 0x6e,
0x00, 0x00, 0x00, 0x00, 0x09, 0x00, 0x00, 0x00, 0x0c, 0x00, 0x00, 0x00, 0x10, 0x00, 0x03, 0x00,
0x04, 0x00, 0x00, 0x00, 0x07, 0x00, 0x00, 0x00, 0x03, 0x00, 0x03, 0x00, 0x02, 0x00, 0x00, 0x00,
0xc2, 0x01, 0x00, 0x00, 0x04, 0x00, 0x09, 0x00, 0x47, 0x4c, 0x5f, 0x41, 0x52, 0x42, 0x5f, 0x73,
0x65, 0x70, 0x61, 0x72, 0x61, 0x74, 0x65, 0x5f, 0x73, 0x68, 0x61, 0x64, 0x65, 0x72, 0x5f, 0x6f,
0x62, 0x6a, 0x65, 0x63, 0x74, 0x73, 0x00, 0x00, 0x04, 0x00, 0x0a, 0x00, 0x47, 0x4c, 0x5f, 0x47,
0x4f, 0x4f, 0x47, 0x4c, 0x45, 0x5f, 0x63, 0x70, 0x70, 0x5f, 0x73, 0x74, 0x79, 0x6c, 0x65, 0x5f,
0x6c, 0x69, 0x6e, 0x65, 0x5f, 0x64, 0x69, 0x72, 0x65, 0x63, 0x74, 0x69, 0x76, 0x65, 0x00, 0x00,
0x04, 0x00, 0x08, 0x00, 0x47, 0x4c, 0x5f, 0x47, 0x4f, 0x4f, 0x47, 0x4c, 0x45, 0x5f, 0x69, 0x6e,
0x63, 0x6c, 0x75, 0x64, 0x65, 0x5f, 0x64, 0x69, 0x72, 0x65, 0x63, 0x74, 0x69, 0x76, 0x65, 0x00,
0x05, 0x00, 0x04, 0x00, 0x04, 0x00, 0x00, 0x00, 0x6d, 0x61, 0x69, 0x6e, 0x00, 0x00, 0x00, 0x00,
0x05, 0x00, 0x05, 0x00, 0x09, 0x00, 0x00, 0x00, 0x6f, 0x75, 0x74, 0x43, 0x6f, 0x6c, 0x6f, 0x72,
0x00, 0x00, 0x00, 0x00, 0x05, 0x00, 0x05, 0x00, 0x0c, 0x00, 0x00, 0x00, 0x66, 0x72, 0x61, 0x67,
0x43, 0x6f, 0x6c, 0x6f, 0x72, 0x00, 0x00, 0x00, 0x47, 0x00, 0x04, 0x00, 0x09, 0x00, 0x00, 0x00,
0x1e, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x47, 0x00, 0x04, 0x00, 0x0c, 0x00, 0x00, 0x00,
0x1e, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x13, 0x00, 0x02, 0x00, 0x02, 0x00, 0x00, 0x00,
0x21, 0x00, 0x03, 0x00, 0x03, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x16, 0x00, 0x03, 0x00,
0x06, 0x00, 0x00, 0x00, 0x20, 0x00, 0x00, 0x00, 0x17, 0x00, 0x04, 0x00, 0x07, 0x00, 0x00, 0x00,
0x06, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x20, 0x00, 0x04, 0x00, 0x08, 0x00, 0x00, 0x00,
0x03, 0x00, 0x00, 0x00, 0x07, 0x00, 0x00, 0x00, 0x3b, 0x00, 0x04, 0x00, 0x08, 0x00, 0x00, 0x00,
0x09, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0x17, 0x00, 0x04, 0x00, 0x0a, 0x00, 0x00, 0x00,
0x06, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0x20, 0x00, 0x04, 0x00, 0x0b, 0x00, 0x00, 0x00,
0x01, 0x00, 0x00, 0x00, 0x0a, 0x00, 0x00, 0x00, 0x3b, 0x00, 0x04, 0x00, 0x0b, 0x00, 0x00, 0x00,
0x0c, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x2b, 0x00, 0x04, 0x00, 0x06, 0x00, 0x00, 0x00,
0x0e, 0x00, 0x00, 0x00, 0x00, 0x00, 0x80, 0x3f, 0x36, 0x00, 0x05, 0x00, 0x02, 0x00, 0x00, 0x00,
0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0xf8, 0x00, 0x02, 0x00,
0x05, 0x00, 0x00, 0x00, 0x3d, 0x00, 0x04, 0x00, 0x0a, 0x00, 0x00, 0x00, 0x0d, 0x00, 0x00, 0x00,
0x0c, 0x00, 0x00, 0x00, 0x51, 0x00, 0x05, 0x00, 0x06, 0x00, 0x00, 0x00, 0x0f, 0x00, 0x00, 0x00,
0x0d, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x51, 0x00, 0x05, 0x00, 0x06, 0x00, 0x00, 0x00,
0x10, 0x00, 0x00, 0x00, 0x0d, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x51, 0x00, 0x05, 0x00,
0x06, 0x00, 0x00, 0x00, 0x11, 0x00, 0x00, 0x00, 0x0d, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00,
0x50, 0x00, 0x07, 0x00, 0x07, 0x00, 0x00, 0x00, 0x12, 0x00, 0x00, 0x00, 0x0f, 0x00, 0x00, 0x00,
0x10, 0x00, 0x00, 0x00, 0x11, 0x00, 0x00, 0x00, 0x0e, 0x00, 0x00, 0x00, 0x3e, 0x00, 0x03, 0x00,
0x09, 0x00, 0x00, 0x00, 0x12, 0x00, 0x00, 0x00, 0xfd, 0x00, 0x01, 0x00, 0x38, 0x00, 0x01, 0x00]
}

const fn vertex_shader() -> [u8; 1540]{
    [0x03, 0x02, 0x23, 0x07, 0x00, 0x00, 0x01, 0x00, 0x0a, 0x00, 0x0d, 0x00, 0x36, 0x00, 0x00, 0x00,
0x00, 0x00, 0x00, 0x00, 0x11, 0x00, 0x02, 0x00, 0x01, 0x00, 0x00, 0x00, 0x0b, 0x00, 0x06, 0x00,
0x01, 0x00, 0x00, 0x00, 0x47, 0x4c, 0x53, 0x4c, 0x2e, 0x73, 0x74, 0x64, 0x2e, 0x34, 0x35, 0x30,
0x00, 0x00, 0x00, 0x00, 0x0e, 0x00, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00,
0x0f, 0x00, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x6d, 0x61, 0x69, 0x6e,
0x00, 0x00, 0x00, 0x00, 0x22, 0x00, 0x00, 0x00, 0x26, 0x00, 0x00, 0x00, 0x31, 0x00, 0x00, 0x00,
0x03, 0x00, 0x03, 0x00, 0x02, 0x00, 0x00, 0x00, 0xc2, 0x01, 0x00, 0x00, 0x04, 0x00, 0x09, 0x00,
0x47, 0x4c, 0x5f, 0x41, 0x52, 0x42, 0x5f, 0x73, 0x65, 0x70, 0x61, 0x72, 0x61, 0x74, 0x65, 0x5f,
0x73, 0x68, 0x61, 0x64, 0x65, 0x72, 0x5f, 0x6f, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x73, 0x00, 0x00,
0x04, 0x00, 0x0a, 0x00, 0x47, 0x4c, 0x5f, 0x47, 0x4f, 0x4f, 0x47, 0x4c, 0x45, 0x5f, 0x63, 0x70,
0x70, 0x5f, 0x73, 0x74, 0x79, 0x6c, 0x65, 0x5f, 0x6c, 0x69, 0x6e, 0x65, 0x5f, 0x64, 0x69, 0x72,
0x65, 0x63, 0x74, 0x69, 0x76, 0x65, 0x00, 0x00, 0x04, 0x00, 0x08, 0x00, 0x47, 0x4c, 0x5f, 0x47,
0x4f, 0x4f, 0x47, 0x4c, 0x45, 0x5f, 0x69, 0x6e, 0x63, 0x6c, 0x75, 0x64, 0x65, 0x5f, 0x64, 0x69,
0x72, 0x65, 0x63, 0x74, 0x69, 0x76, 0x65, 0x00, 0x05, 0x00, 0x04, 0x00, 0x04, 0x00, 0x00, 0x00,
0x6d, 0x61, 0x69, 0x6e, 0x00, 0x00, 0x00, 0x00, 0x05, 0x00, 0x05, 0x00, 0x0c, 0x00, 0x00, 0x00,
0x70, 0x6f, 0x73, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x00, 0x00, 0x00, 0x05, 0x00, 0x04, 0x00,
0x17, 0x00, 0x00, 0x00, 0x63, 0x6f, 0x6c, 0x6f, 0x72, 0x73, 0x00, 0x00, 0x05, 0x00, 0x06, 0x00,
0x20, 0x00, 0x00, 0x00, 0x67, 0x6c, 0x5f, 0x50, 0x65, 0x72, 0x56, 0x65, 0x72, 0x74, 0x65, 0x78,
0x00, 0x00, 0x00, 0x00, 0x06, 0x00, 0x06, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
0x67, 0x6c, 0x5f, 0x50, 0x6f, 0x73, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x00, 0x06, 0x00, 0x07, 0x00,
0x20, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x67, 0x6c, 0x5f, 0x50, 0x6f, 0x69, 0x6e, 0x74,
0x53, 0x69, 0x7a, 0x65, 0x00, 0x00, 0x00, 0x00, 0x06, 0x00, 0x07, 0x00, 0x20, 0x00, 0x00, 0x00,
0x02, 0x00, 0x00, 0x00, 0x67, 0x6c, 0x5f, 0x43, 0x6c, 0x69, 0x70, 0x44, 0x69, 0x73, 0x74, 0x61,
0x6e, 0x63, 0x65, 0x00, 0x06, 0x00, 0x07, 0x00, 0x20, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00,
0x67, 0x6c, 0x5f, 0x43, 0x75, 0x6c, 0x6c, 0x44, 0x69, 0x73, 0x74, 0x61, 0x6e, 0x63, 0x65, 0x00,
0x05, 0x00, 0x03, 0x00, 0x22, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x05, 0x00, 0x06, 0x00,
0x26, 0x00, 0x00, 0x00, 0x67, 0x6c, 0x5f, 0x56, 0x65, 0x72, 0x74, 0x65, 0x78, 0x49, 0x6e, 0x64,
0x65, 0x78, 0x00, 0x00, 0x05, 0x00, 0x05, 0x00, 0x31, 0x00, 0x00, 0x00, 0x66, 0x72, 0x61, 0x67,
0x43, 0x6f, 0x6c, 0x6f, 0x72, 0x00, 0x00, 0x00, 0x48, 0x00, 0x05, 0x00, 0x20, 0x00, 0x00, 0x00,
0x00, 0x00, 0x00, 0x00, 0x0b, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x48, 0x00, 0x05, 0x00,
0x20, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x0b, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00,
0x48, 0x00, 0x05, 0x00, 0x20, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x0b, 0x00, 0x00, 0x00,
0x03, 0x00, 0x00, 0x00, 0x48, 0x00, 0x05, 0x00, 0x20, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00,
0x0b, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x47, 0x00, 0x03, 0x00, 0x20, 0x00, 0x00, 0x00,
0x02, 0x00, 0x00, 0x00, 0x47, 0x00, 0x04, 0x00, 0x26, 0x00, 0x00, 0x00, 0x0b, 0x00, 0x00, 0x00,
0x2a, 0x00, 0x00, 0x00, 0x47, 0x00, 0x04, 0x00, 0x31, 0x00, 0x00, 0x00, 0x1e, 0x00, 0x00, 0x00,
0x00, 0x00, 0x00, 0x00, 0x13, 0x00, 0x02, 0x00, 0x02, 0x00, 0x00, 0x00, 0x21, 0x00, 0x03, 0x00,
0x03, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x16, 0x00, 0x03, 0x00, 0x06, 0x00, 0x00, 0x00,
0x20, 0x00, 0x00, 0x00, 0x17, 0x00, 0x04, 0x00, 0x07, 0x00, 0x00, 0x00, 0x06, 0x00, 0x00, 0x00,
0x02, 0x00, 0x00, 0x00, 0x15, 0x00, 0x04, 0x00, 0x08, 0x00, 0x00, 0x00, 0x20, 0x00, 0x00, 0x00,
0x00, 0x00, 0x00, 0x00, 0x2b, 0x00, 0x04, 0x00, 0x08, 0x00, 0x00, 0x00, 0x09, 0x00, 0x00, 0x00,
0x03, 0x00, 0x00, 0x00, 0x1c, 0x00, 0x04, 0x00, 0x0a, 0x00, 0x00, 0x00, 0x07, 0x00, 0x00, 0x00,
0x09, 0x00, 0x00, 0x00, 0x20, 0x00, 0x04, 0x00, 0x0b, 0x00, 0x00, 0x00, 0x06, 0x00, 0x00, 0x00,
0x0a, 0x00, 0x00, 0x00, 0x3b, 0x00, 0x04, 0x00, 0x0b, 0x00, 0x00, 0x00, 0x0c, 0x00, 0x00, 0x00,
0x06, 0x00, 0x00, 0x00, 0x2b, 0x00, 0x04, 0x00, 0x06, 0x00, 0x00, 0x00, 0x0d, 0x00, 0x00, 0x00,
0x00, 0x00, 0x00, 0x00, 0x2b, 0x00, 0x04, 0x00, 0x06, 0x00, 0x00, 0x00, 0x0e, 0x00, 0x00, 0x00,
0x00, 0x00, 0x00, 0xbf, 0x2c, 0x00, 0x05, 0x00, 0x07, 0x00, 0x00, 0x00, 0x0f, 0x00, 0x00, 0x00,
0x0d, 0x00, 0x00, 0x00, 0x0e, 0x00, 0x00, 0x00, 0x2b, 0x00, 0x04, 0x00, 0x06, 0x00, 0x00, 0x00,
0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x3f, 0x2c, 0x00, 0x05, 0x00, 0x07, 0x00, 0x00, 0x00,
0x11, 0x00, 0x00, 0x00, 0x10, 0x00, 0x00, 0x00, 0x10, 0x00, 0x00, 0x00, 0x2c, 0x00, 0x05, 0x00,
0x07, 0x00, 0x00, 0x00, 0x12, 0x00, 0x00, 0x00, 0x0e, 0x00, 0x00, 0x00, 0x10, 0x00, 0x00, 0x00,
0x2c, 0x00, 0x06, 0x00, 0x0a, 0x00, 0x00, 0x00, 0x13, 0x00, 0x00, 0x00, 0x0f, 0x00, 0x00, 0x00,
0x11, 0x00, 0x00, 0x00, 0x12, 0x00, 0x00, 0x00, 0x17, 0x00, 0x04, 0x00, 0x14, 0x00, 0x00, 0x00,
0x06, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0x1c, 0x00, 0x04, 0x00, 0x15, 0x00, 0x00, 0x00,
0x14, 0x00, 0x00, 0x00, 0x09, 0x00, 0x00, 0x00, 0x20, 0x00, 0x04, 0x00, 0x16, 0x00, 0x00, 0x00,
0x06, 0x00, 0x00, 0x00, 0x15, 0x00, 0x00, 0x00, 0x3b, 0x00, 0x04, 0x00, 0x16, 0x00, 0x00, 0x00,
0x17, 0x00, 0x00, 0x00, 0x06, 0x00, 0x00, 0x00, 0x2b, 0x00, 0x04, 0x00, 0x06, 0x00, 0x00, 0x00,
0x18, 0x00, 0x00, 0x00, 0x00, 0x00, 0x80, 0x3f, 0x2c, 0x00, 0x06, 0x00, 0x14, 0x00, 0x00, 0x00,
0x19, 0x00, 0x00, 0x00, 0x18, 0x00, 0x00, 0x00, 0x0d, 0x00, 0x00, 0x00, 0x0d, 0x00, 0x00, 0x00,
0x2c, 0x00, 0x06, 0x00, 0x14, 0x00, 0x00, 0x00, 0x1a, 0x00, 0x00, 0x00, 0x0d, 0x00, 0x00, 0x00,
0x18, 0x00, 0x00, 0x00, 0x0d, 0x00, 0x00, 0x00, 0x2c, 0x00, 0x06, 0x00, 0x14, 0x00, 0x00, 0x00,
0x1b, 0x00, 0x00, 0x00, 0x0d, 0x00, 0x00, 0x00, 0x0d, 0x00, 0x00, 0x00, 0x18, 0x00, 0x00, 0x00,
0x2c, 0x00, 0x06, 0x00, 0x15, 0x00, 0x00, 0x00, 0x1c, 0x00, 0x00, 0x00, 0x19, 0x00, 0x00, 0x00,
0x1a, 0x00, 0x00, 0x00, 0x1b, 0x00, 0x00, 0x00, 0x17, 0x00, 0x04, 0x00, 0x1d, 0x00, 0x00, 0x00,
0x06, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x2b, 0x00, 0x04, 0x00, 0x08, 0x00, 0x00, 0x00,
0x1e, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x1c, 0x00, 0x04, 0x00, 0x1f, 0x00, 0x00, 0x00,
0x06, 0x00, 0x00, 0x00, 0x1e, 0x00, 0x00, 0x00, 0x1e, 0x00, 0x06, 0x00, 0x20, 0x00, 0x00, 0x00,
0x1d, 0x00, 0x00, 0x00, 0x06, 0x00, 0x00, 0x00, 0x1f, 0x00, 0x00, 0x00, 0x1f, 0x00, 0x00, 0x00,
0x20, 0x00, 0x04, 0x00, 0x21, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0x20, 0x00, 0x00, 0x00,
0x3b, 0x00, 0x04, 0x00, 0x21, 0x00, 0x00, 0x00, 0x22, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00,
0x15, 0x00, 0x04, 0x00, 0x23, 0x00, 0x00, 0x00, 0x20, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00,
0x2b, 0x00, 0x04, 0x00, 0x23, 0x00, 0x00, 0x00, 0x24, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
0x20, 0x00, 0x04, 0x00, 0x25, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x23, 0x00, 0x00, 0x00,
0x3b, 0x00, 0x04, 0x00, 0x25, 0x00, 0x00, 0x00, 0x26, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00,
0x20, 0x00, 0x04, 0x00, 0x28, 0x00, 0x00, 0x00, 0x06, 0x00, 0x00, 0x00, 0x07, 0x00, 0x00, 0x00,
0x20, 0x00, 0x04, 0x00, 0x2e, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0x1d, 0x00, 0x00, 0x00,
0x20, 0x00, 0x04, 0x00, 0x30, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0x14, 0x00, 0x00, 0x00,
0x3b, 0x00, 0x04, 0x00, 0x30, 0x00, 0x00, 0x00, 0x31, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00,
0x20, 0x00, 0x04, 0x00, 0x33, 0x00, 0x00, 0x00, 0x06, 0x00, 0x00, 0x00, 0x14, 0x00, 0x00, 0x00,
0x36, 0x00, 0x05, 0x00, 0x02, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
0x03, 0x00, 0x00, 0x00, 0xf8, 0x00, 0x02, 0x00, 0x05, 0x00, 0x00, 0x00, 0x3e, 0x00, 0x03, 0x00,
0x0c, 0x00, 0x00, 0x00, 0x13, 0x00, 0x00, 0x00, 0x3e, 0x00, 0x03, 0x00, 0x17, 0x00, 0x00, 0x00,
0x1c, 0x00, 0x00, 0x00, 0x3d, 0x00, 0x04, 0x00, 0x23, 0x00, 0x00, 0x00, 0x27, 0x00, 0x00, 0x00,
0x26, 0x00, 0x00, 0x00, 0x41, 0x00, 0x05, 0x00, 0x28, 0x00, 0x00, 0x00, 0x29, 0x00, 0x00, 0x00,
0x0c, 0x00, 0x00, 0x00, 0x27, 0x00, 0x00, 0x00, 0x3d, 0x00, 0x04, 0x00, 0x07, 0x00, 0x00, 0x00,
0x2a, 0x00, 0x00, 0x00, 0x29, 0x00, 0x00, 0x00, 0x51, 0x00, 0x05, 0x00, 0x06, 0x00, 0x00, 0x00,
0x2b, 0x00, 0x00, 0x00, 0x2a, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x51, 0x00, 0x05, 0x00,
0x06, 0x00, 0x00, 0x00, 0x2c, 0x00, 0x00, 0x00, 0x2a, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00,
0x50, 0x00, 0x07, 0x00, 0x1d, 0x00, 0x00, 0x00, 0x2d, 0x00, 0x00, 0x00, 0x2b, 0x00, 0x00, 0x00,
0x2c, 0x00, 0x00, 0x00, 0x0d, 0x00, 0x00, 0x00, 0x18, 0x00, 0x00, 0x00, 0x41, 0x00, 0x05, 0x00,
0x2e, 0x00, 0x00, 0x00, 0x2f, 0x00, 0x00, 0x00, 0x22, 0x00, 0x00, 0x00, 0x24, 0x00, 0x00, 0x00,
0x3e, 0x00, 0x03, 0x00, 0x2f, 0x00, 0x00, 0x00, 0x2d, 0x00, 0x00, 0x00, 0x3d, 0x00, 0x04, 0x00,
0x23, 0x00, 0x00, 0x00, 0x32, 0x00, 0x00, 0x00, 0x26, 0x00, 0x00, 0x00, 0x41, 0x00, 0x05, 0x00,
0x33, 0x00, 0x00, 0x00, 0x34, 0x00, 0x00, 0x00, 0x17, 0x00, 0x00, 0x00, 0x32, 0x00, 0x00, 0x00,
0x3d, 0x00, 0x04, 0x00, 0x14, 0x00, 0x00, 0x00, 0x35, 0x00, 0x00, 0x00, 0x34, 0x00, 0x00, 0x00,
0x3e, 0x00, 0x03, 0x00, 0x31, 0x00, 0x00, 0x00, 0x35, 0x00, 0x00, 0x00, 0xfd, 0x00, 0x01, 0x00,
0x38, 0x00, 0x01, 0x00]
}
