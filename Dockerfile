# Базовый образ с поддержкой Rust
FROM rust:latest

# Установка необходимых инструментов для кросс-компиляции под Windows
RUN apt-get update && \
    apt-get install -y mingw-w64 && \
    rustup target add x86_64-pc-windows-gnu

# Устанавливаем qemu-user для запуска бинарников других архитектур
RUN apt-get install -y qemu-user qemu-user-static

# Создаем директорию для проекта
WORKDIR /usr/src/myapp

# Копируем файлы проекта
COPY . .

# Сборка проекта для Windows
RUN cargo build --release --target=x86_64-pc-windows-gnu --verbose

# Используем qemu для запуска скомпилированного бинарника
CMD ["qemu-x86_64", "target/x86_64-pc-windows-gnu/release/test2.exe"]
