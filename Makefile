# Makefile

# 构建并运行应用程序，带环境变量
run:
	@cargo run --bin editor

# 调试模式
debug:
	@GTK_DEBUG=interactive cargo run --bin editor
