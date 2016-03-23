NAME=CompositeOperations
VERSION=0.8.6

GH_REPO=stanislaw/CompositeOperations
ZIPBALL_URL=https://github.com/$(GH_REPO)/releases/download/$(VERSION)/CompositeOperations-iOS.zip

### Paths

COMPONENTS_BUILD_CACHE_PATH ?= $(HOME)/Library/Caches/Components
COMPONENTS_INSTALL_PATH ?= ./Components

COMPONENT_BUILD_PATH=$(COMPONENTS_BUILD_CACHE_PATH)/$(NAME)
COMPONENT_SOURCE_PATH=$(COMPONENT_BUILD_PATH)/$(NAME)-$(VERSION)
COMPONENT_FRAMEWORK_PATH=$(COMPONENT_SOURCE_PATH)/$(NAME).framework

COMPONENT_INSTALL_PATH=$(COMPONENTS_INSTALL_PATH)/$(NAME)

ZIPBALL_PATH=$(COMPONENT_BUILD_PATH)/$(NAME)-$(VERSION).zip

### Targets

.PHONY: install update uninstall clean prepare purge

install: $(COMPONENT_INSTALL_PATH)

uninstall:
	rm -rf $(COMPONENT_INSTALL_PATH)

update: uninstall install

clean:
	rm -rf $(COMPONENT_SOURCE_PATH)
	rm -rf $(ZIPBALL_PATH)

purge: uninstall clean

### Artefacts

$(COMPONENT_INSTALL_PATH): $(COMPONENT_SOURCE_PATH)
	mkdir -p $(COMPONENT_INSTALL_PATH)
	cp -Rv $(COMPONENT_FRAMEWORK_PATH) $(COMPONENT_INSTALL_PATH)

$(COMPONENT_SOURCE_PATH): $(ZIPBALL_PATH)
	unzip $(ZIPBALL_PATH) -d $(COMPONENT_SOURCE_PATH)

$(ZIPBALL_PATH):
	mkdir -p $(COMPONENT_BUILD_PATH)
	wget --no-use-server-timestamps $(ZIPBALL_URL) -O $(ZIPBALL_PATH)

