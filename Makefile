# 构建并运行应用程序，带环境变量
run:
	@cargo run --bin editor

# 调试模式
debug:
	@GTK_DEBUG=interactive cargo run --bin editor


.PHONY: setup
setup:
ifndef SCHEMA
	$(error SCHEMA is undefined. Use 'make setup SCHEMA=filename.gschema.xml')
endif
	@if [ ! -d "$(HOME)/.local/share/glib-2.0/schemas" ]; then \
		echo "Creating directory $(HOME)/.local/share/glib-2.0/schemas"; \
		mkdir -p $(HOME)/.local/share/glib-2.0/schemas; \
	else \
		echo "Directory $(HOME)/.local/share/glib-2.0/schemas already exists"; \
	fi
	@cp $(SCHEMA) $(HOME)/.local/share/glib-2.0/schemas/
	@glib-compile-schemas $(HOME)/.local/share/glib-2.0/schemas/
	@echo "Schema $(SCHEMA) installed and compiled."

