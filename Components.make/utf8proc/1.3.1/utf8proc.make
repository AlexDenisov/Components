NAME=utf8proc
VERSION=1.3.1
GH_REPO=JuliaLang/utf8proc

### URLs

COMPONENT_TARBALL_URL=https://github.com/$(GH_REPO)/archive/v$(VERSION).tar.gz

### Paths

COMPONENTS_BUILD_CACHE_PATH ?= $(HOME)/Library/Caches/Components
COMPONENTS_INSTALL_PATH ?= ./Components

COMPONENT_BUILD_PATH=$(COMPONENTS_BUILD_CACHE_PATH)/$(NAME)
COMPONENT_TARBALL_PATH=$(COMPONENT_BUILD_PATH)/$(NAME)-$(VERSION).tar.gz
COMPONENT_SOURCE_PATH=$(COMPONENT_BUILD_PATH)/$(NAME)-$(VERSION)
COMPONENT_ARTEFACT_PATH=$(COMPONENT_SOURCE_PATH)/$(NAME)-$(VERSION)

COMPONENT_INSTALL_PATH=$(COMPONENTS_INSTALL_PATH)/$(NAME)

### Targets

.PHONY: install update uninstall clean prepare purge

install: $(COMPONENT_INSTALL_PATH)

uninstall:
	rm -rf $(COMPONENT_INSTALL_PATH)

update: uninstall install

clean:
	rm -rf $(COMPONENT_SOURCE_PATH)
	rm -rf $(COMPONENT_TARBALL_PATH)

purge: uninstall clean

### Artefacts

$(COMPONENT_INSTALL_PATH): $(COMPONENT_SOURCE_PATH)
	mkdir -p $(COMPONENT_INSTALL_PATH)

	cp -Rv $(COMPONENT_ARTEFACT_PATH)/utf8proc*\.[hc] $(COMPONENT_INSTALL_PATH)

$(COMPONENT_SOURCE_PATH): $(COMPONENT_TARBALL_PATH)
	mkdir -p $(COMPONENT_SOURCE_PATH)
	tar xzf $(COMPONENT_TARBALL_PATH) --directory ${COMPONENT_SOURCE_PATH}

$(COMPONENT_TARBALL_PATH):
	mkdir -p $(COMPONENT_BUILD_PATH)
	wget --no-use-server-timestamps $(COMPONENT_TARBALL_URL) -O $(COMPONENT_TARBALL_PATH)

