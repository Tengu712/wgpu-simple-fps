# Coding Convension

## Use

外部モジュールのオブジェクトは必ず具体的に`use`する。
ただし、外部モジュールの関数及び定数はその所属するモジュールを必ず具体的に`use`する。

```rs
// NG
fn run() -> Result<(), Box<dyn std::error::Error>> {}
fn main() {
    if let Err(_) = run() {
        std::process::exit(1);
    }
}

// OK
use std::{error::Error, process};
fn run() -> Result<(), Box<dyn Error>> {}
fn main() {
    if let Err(_) = run() {
        process::exit(1);
    }
}
```

## Variable Name

なるべく変数名は省略しない。
自由度が高いが故にカオス化することを防ぐためである。

```rs
// NG
let shader = device.create_shader_module(ShaderModuleDescriptor {
    label: None,
    source: ShaderSource::Wgsl(Cow::from(SHADER)),
});

// OK
let shader_module = device.create_shader_module(ShaderModuleDescriptor {
    label: None,
    source: ShaderSource::Wgsl(Cow::from(SHADER)),
});
```

## Temporary Variable

なるべく一時変数の束縛を避ける。
一時変数内の一時変数のドロップを防ぐためである。

```rs
// NG
let instance_descriptor = InstanceDescriptor {
    backends: Backends::all(),
    ..Default::default()
};
let instance = Instance::new(instance_descriptor);

// OK
let instance = Instance::new(InstanceDescriptor {
    backends: Backends::all(),
    ..Default::default()
});
```
