use ash::{
    version::DeviceV1_0,
    vk::{Pipeline, PipelineLayout},
    Device,
};

pub struct ManagedPipeline<'a> {
    device: &'a Device,
    pipeline_layout: PipelineLayout,
    pipeline_raw: Pipeline,
}

impl<'a> ManagedPipeline<'a> {
    pub fn new(
        device: &'a Device,
        pipeline_layout: PipelineLayout,
        pipeline_raw: Pipeline,
    ) -> ManagedPipeline<'a> {
        ManagedPipeline {
            device,
            pipeline_layout,
            pipeline_raw,
        }
    }

    pub fn get_pipeline_raw(&self) -> Pipeline {
        self.pipeline_raw
    }
}

impl Drop for ManagedPipeline<'_> {
    fn drop(&mut self) {
        unsafe {
            self.device
                .destroy_pipeline_layout(self.pipeline_layout, None);
            trace!("PipelineLayout was destroyed");
            self.device.destroy_pipeline(self.pipeline_raw, None);
            trace!("Pipeline was destroyed");
        }
    }
}
