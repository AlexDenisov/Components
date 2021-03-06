NAME=TPCircularBuffer
VERSION=master
GH_REPO=michaeltyson/TPCircularBuffer

### URLs

COMPONENT_ZIPBALL_URL=https://github.com/$(GH_REPO)/archive/$(VERSION).zip

### Paths

COMPONENTS_BUILD_CACHE_PATH ?= $(HOME)/Library/Caches/Components
COMPONENTS_INSTALL_PATH ?= ./Components

COMPONENT_BUILD_PATH=$(COMPONENTS_BUILD_CACHE_PATH)/$(NAME)
COMPONENT_ZIPBALL_PATH=$(COMPONENT_BUILD_PATH)/$(NAME)-$(VERSION).zip
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
	rm -rf $(COMPONENT_ZIPBALL_PATH)

purge: uninstall clean

### Artefacts

$(COMPONENT_INSTALL_PATH): $(COMPONENT_SOURCE_PATH)
	mkdir -p $(COMPONENT_INSTALL_PATH)

	cp -Rv $(COMPONENT_ARTEFACT_PATH)/TPCircularBuffer.[ch] $(COMPONENT_INSTALL_PATH)
	cp -Rv $(COMPONENT_ARTEFACT_PATH)/TPCircularBuffer+AudioBufferList.[ch] $(COMPONENT_INSTALL_PATH)

$(COMPONENT_SOURCE_PATH): $(COMPONENT_ZIPBALL_PATH)
	unzip $(COMPONENT_ZIPBALL_PATH) -d $(COMPONENT_SOURCE_PATH)

$(COMPONENT_ZIPBALL_PATH):
	mkdir -p $(COMPONENT_BUILD_PATH)
	wget --no-use-server-timestamps $(COMPONENT_ZIPBALL_URL) -O $(COMPONENT_ZIPBALL_PATH)

