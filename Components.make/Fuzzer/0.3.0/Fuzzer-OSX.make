NAME=Fuzzer-OSX
VERSION=0.3.0

GH_REPO=AlexDenisov/Fuzzer
ZIPBALL_URL=https://github.com/$(GH_REPO)/releases/download/$(VERSION)/$(NAME)-$(VERSION).zip

### Paths

COMPONENTS_BUILD_CACHE_PATH ?= $(HOME)/Library/Caches/Components
COMPONENTS_INSTALL_PATH ?= ./Components

COMPONENT_BUILD_PATH=$(COMPONENTS_BUILD_CACHE_PATH)/$(NAME)
COMPONENT_SOURCE_PATH=$(COMPONENT_BUILD_PATH)/$(NAME)-$(VERSION)
COMPONENT_FRAMEWORK_PATH=$(COMPONENT_SOURCE_PATH)/Fuzzer.framework

COMPONENT_INSTALL_PATH=$(COMPONENTS_INSTALL_PATH)/$(NAME)

COMPONENT_ZIPBALL_PATH=$(COMPONENT_BUILD_PATH)/$(NAME)-$(VERSION).zip

### Targets

.PHONY: install update uninstall clean prepare purge

install: $(COMPONENT_INSTALL_PATH)

uninstall:
	rm -rf $(COMPONENT_INSTALL_PATH)

update: uninstall install

clean:
	rm -rf $(COMPONENT_SOURCE_PATH)
	rm -rf $(COMPONENT_ZIPBALL_PATH)

purge: uninstall clean

### Artefacts

$(COMPONENT_INSTALL_PATH): $(COMPONENT_FRAMEWORK_PATH)
	mkdir -p $(COMPONENT_INSTALL_PATH)
	cp -Rv $(COMPONENT_FRAMEWORK_PATH) $(COMPONENT_INSTALL_PATH)

$(COMPONENT_FRAMEWORK_PATH): $(COMPONENT_ZIPBALL_PATH)
	unzip $(COMPONENT_ZIPBALL_PATH) -d $(COMPONENT_SOURCE_PATH)
	
	# Unzipping touches $(COMPONENT_BUILD_PATH)
	# so we must touch zipball and then source path to restore logical order
	touch $(COMPONENT_ZIPBALL_PATH)
	touch $(COMPONENT_FRAMEWORK_PATH)

$(COMPONENT_ZIPBALL_PATH):
	mkdir -p $(COMPONENT_BUILD_PATH)

	wget --no-use-server-timestamps $(ZIPBALL_URL) -O $(COMPONENT_ZIPBALL_PATH)

