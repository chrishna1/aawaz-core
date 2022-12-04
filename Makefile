BUILDDIR      ?= artifacts
PROJECT_ROOT   ?= /home/insoluble/Workspace/projects/aawaz-core

# dev: serve-a-sample-blog serve-backend serve-frontend

# serve-frontend: serve-frontend-plugin serve-frontend-plugin-helper

# dev related stuff..
serve-a-sample-blog:
	cd /home/insoluble/Workspace/projects/aawaz-test-blogs/hugoBasicExample && hugo server -t hugo-vitae

serve-backend:
	cd backend && cargo watch -x run

serve-frontend-plugin:
	cd frontend/plugin && npm run dev-widget

serve-frontend-plugin-helper:
	cd frontend/plugin && npm run dev-embed

frontend-helper:
	cd frontend/plugin/dist && http-server -p 8081


# test related stuff
test: test-frontend test-backend

test-frontend:
	@echo "Write test first :)"

test-backend:
	cd $(PROJECT_ROOT) && cd backend && cargo test -- --test-threads 1


# build related stuff..

build: build-sample-blog build-frontend build-backend

build-sample-blog:
	cd $(PROJECT_ROOT) && cd /home/insoluble/Workspace/projects/aawaz-test-blogs/hugoBasicExample && hugo && echo $(PROJECT_ROOT) && cp -r public/ $(PROJECT_ROOT)/$(BUILDDIR)/blog/

build-frontend:
	cd $(PROJECT_ROOT) && cd frontend/plugin && npm run dev-widget-prod && npm run dev-embed-prod && cp -r dist/ $(PROJECT_ROOT)/$(BUILDDIR)/frontend/

build-backend:
	cd $(PROJECT_ROOT) && cd backend && cargo build -r && cp -r target/release/aawaz $(PROJECT_ROOT)/$(BUILDDIR)/backend/


# deploy related stuff..

deploy-sample-blog:
	cd $(PROJECT_ROOT) && rsync-copy artifacts/blog/ hackadda:/home/krishna/blog.hackadda.com

deploy:
	ansible-playbook main.yml --tags=copy

deploy-frontend:
	ansible-playbook main.yml --tags=frontend

deploy-backend:
	ansible-playbook main.yml --tags=backend



# .PHONY: clean

# clean:
# 	-rm -rf $(BUILDDIR)/*
