mod cpu;
mod devices;

const MEM_SIZE: usize = 1024 * 1024 * 1024; // 1GB

fn main() {
    let mut cpu = cpu::Cpu::new(MEM_SIZE);
    cpu.run();
}
