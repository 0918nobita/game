use ash::{
    version::DeviceV1_0,
    vk::{Pipeline, PipelineLayout},
    Device,
};

pub struct ManagedPipeline<'a> {
    device_raw: &'a Device,
    pipeline_layout: PipelineLayout,
    pipeline: Pipeline,
}

impl<'a> ManagedPipeline<'a> {
    pub fn new(
        device_raw: &'a Device,
        pipeline_layout: PipelineLayout,
        pipeline: Pipeline,
    ) -> ManagedPipeline<'a> {
        ManagedPipeline {
            device_raw,
            pipeline_layout,
            pipeline,
        }
    }
}

impl Drop for ManagedPipeline<'_> {
    fn drop(&mut self) {
        unsafe {
            self.device_raw
                .destroy_pipeline_layout(self.pipeline_layout, None);
            trace!("PipelineLayout was destroyed");
            self.device_raw.destroy_pipeline(self.pipeline, None);
            trace!("Pipeline was destroyed");
        }
    }
}
