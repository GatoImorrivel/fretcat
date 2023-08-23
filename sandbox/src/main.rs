#[hot_lib_reloader::hot_module(dylib = "process")]
mod hot_lib {
    // Reads public no_mangle functions from lib.rs and  generates hot-reloadable
    // wrapper functions with the same signature inside this module.
    // Note that this path relative to the project root (or absolute)
    hot_functions_from_file!("process/src/lib.rs");
}

use std::{fs::File, io::BufReader, path::Path};

use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    FromSample, Sample, SizedSample,
};
use hound::WavReader;

fn main() -> anyhow::Result<()> {
    let stream = stream_setup_for()?;
    stream.play()?;

    // Wait for a keypress to exit
    println!("Press Enter to exit...");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    Ok(())
}

pub fn stream_setup_for() -> Result<cpal::Stream, anyhow::Error>
where
{
    let (_host, device, config) = host_device_setup()?;

    match config.sample_format() {
        cpal::SampleFormat::I8 => make_stream::<i8>(&device, &config.into()),
        cpal::SampleFormat::I16 => make_stream::<i16>(&device, &config.into()),
        cpal::SampleFormat::I32 => make_stream::<i32>(&device, &config.into()),
        cpal::SampleFormat::I64 => make_stream::<i64>(&device, &config.into()),
        cpal::SampleFormat::U8 => make_stream::<u8>(&device, &config.into()),
        cpal::SampleFormat::U16 => make_stream::<u16>(&device, &config.into()),
        cpal::SampleFormat::U32 => make_stream::<u32>(&device, &config.into()),
        cpal::SampleFormat::U64 => make_stream::<u64>(&device, &config.into()),
        cpal::SampleFormat::F32 => make_stream::<f32>(&device, &config.into()),
        cpal::SampleFormat::F64 => make_stream::<f64>(&device, &config.into()),
        sample_format => Err(anyhow::Error::msg(format!(
            "Unsupported sample format '{sample_format}'"
        ))),
    }
}

pub fn host_device_setup(
) -> Result<(cpal::Host, cpal::Device, cpal::SupportedStreamConfig), anyhow::Error> {
    let host = cpal::default_host();

    let device = host
        .default_output_device()
        .ok_or_else(|| anyhow::Error::msg("Default output device is not available"))?;
    println!("Output device : {}", device.name()?);

    let config = device.default_output_config()?;
    println!("Default output config : {:?}", config);

    Ok((host, device, config))
}

struct Input {
    samples: Vec<f32>,
    current_sample: usize,
}

impl Input {
    pub fn new(input: Vec<f32>) -> Self {
        Self {
            samples: input,
            current_sample: 0usize,
        }
    }

    pub fn tick(&mut self) -> f32 {
        if self.current_sample >= self.samples.len() {
            self.current_sample = 0;
        }
        let out = self.samples[self.current_sample];
        self.current_sample += 1;
        out
    }
}

pub fn make_stream<T>(
    device: &cpal::Device,
    config: &cpal::StreamConfig,
) -> Result<cpal::Stream, anyhow::Error>
where
    T: SizedSample + FromSample<f32>,
{
    let num_channels = config.channels as usize;
    let err_fn = |err| eprintln!("Error building output sound stream: {}", err);

    let mut reader = WavReader::open("./H.wav").unwrap();
    let samples = reader
        .samples::<i16>()
        .map(|s| s.unwrap() as f32)
        .collect::<Vec<_>>();

    let mut input = Input::new(samples);

    let stream = device.build_output_stream(
        config,
        move |output: &mut [T], _: &cpal::OutputCallbackInfo| {
            process_frame(output, &mut input, num_channels)
        },
        err_fn,
        None,
    )?;

    Ok(stream)
}

fn process_frame<SampleType>(output: &mut [SampleType], input: &mut Input, num_channels: usize)
where
    SampleType: Sample + FromSample<f32>,
{
    let mut buffer = vec![];
    for i in 0..output.len() {
        buffer.push(input.tick());
    }
    hot_lib::process_sample(&mut buffer);

    for (i, sample) in output.iter_mut().enumerate() {
        *sample = SampleType::from_sample(buffer[i] / 100000.0);
    }
}
