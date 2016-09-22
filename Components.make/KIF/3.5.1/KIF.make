NAME=KIF
VERSION=3.5.1
GH_REPO=kif-framework/KIF

### URLs

COMPONENT_ZIPBALL_URL=https://github.com/$(GH_REPO)/archive/v$(VERSION).zip

### Paths

COMPONENTS_BUILD_CACHE_PATH ?= $(HOME)/Library/Caches/Components
COMPONENTS_INSTALL_PATH ?= ./Components

COMPONENT_BUILD_PATH=$(COMPONENTS_BUILD_CACHE_PATH)/$(NAME)
COMPONENT_ZIPBALL_PATH=$(COMPONENT_BUILD_PATH)/$(NAME)-$(VERSION).zip
COMPONENT_SOURCE_PATH=$(COMPONENT_BUILD_PATH)/$(NAME)-$(VERSION)
COMPONENT_XCODEPROJ_PATH=$(COMPONENT_SOURCE_PATH)/$(NAME)-$(VERSION)/KIF.xcodeproj
COMPONENT_ARTEFACT_PATH=$(COMPONENT_SOURCE_PATH)/Artefacts/KIF
COMPONENT_ARTEFACT_PATH_SIMULATOR=$(COMPONENT_ARTEFACT_PATH)/SIMULATOR
COMPONENT_ARTEFACT_PATH_IPHONE=$(COMPONENT_ARTEFACT_PATH)/IPHONE
COMPONENT_ARTEFACT_PATH_UNIVERSAL=$(COMPONENT_ARTEFACT_PATH)/UNIVERSAL

COMPONENT_INSTALL_PATH=$(COMPONENTS_INSTALL_PATH)/$(NAME)/$(NAME)

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

$(COMPONENT_INSTALL_PATH): | $(COMPONENT_ARTEFACT_PATH)
	mkdir -p $(COMPONENT_INSTALL_PATH)
	cp -Rv $(COMPONENT_ARTEFACT_PATH_UNIVERSAL)/include/KIF/* $(COMPONENT_INSTALL_PATH)
	cp -Rv $(COMPONENT_ARTEFACT_PATH_UNIVERSAL)/libKIF.a $(COMPONENT_INSTALL_PATH)

$(COMPONENT_ARTEFACT_PATH): | $(COMPONENT_SOURCE_PATH)
	xcodebuild \
		ONLY_ACTIVE_ARCH=NO \
		CONFIGURATION_BUILD_DIR=$(COMPONENT_ARTEFACT_PATH_SIMULATOR) \
		-project $(COMPONENT_XCODEPROJ_PATH) \
        -sdk iphonesimulator \
        -destination 'platform=iOS Simulator,name=iPhone 6S Plus,OS=latest' \
        -configuration Release \
		clean build

	xcodebuild \
		ONLY_ACTIVE_ARCH=NO \
		CONFIGURATION_BUILD_DIR=$(COMPONENT_ARTEFACT_PATH_IPHONE) \
		-project $(COMPONENT_XCODEPROJ_PATH) \
        -sdk iphoneos \
        -configuration Release \
		clean build

	mkdir -p $(COMPONENT_ARTEFACT_PATH_UNIVERSAL)

	cp -rv $(COMPONENT_ARTEFACT_PATH_IPHONE)/* $(COMPONENT_ARTEFACT_PATH_UNIVERSAL)

	lipo $(COMPONENT_ARTEFACT_PATH_IPHONE)/libKIF.a $(COMPONENT_ARTEFACT_PATH_SIMULATOR)/libKIF.a -create -output $(COMPONENT_ARTEFACT_PATH_UNIVERSAL)/libKIF.a

$(COMPONENT_SOURCE_PATH): | $(COMPONENT_ZIPBALL_PATH)
	unzip $(COMPONENT_ZIPBALL_PATH) -d $(COMPONENT_SOURCE_PATH)

$(COMPONENT_ZIPBALL_PATH):
	mkdir -p $(COMPONENT_BUILD_PATH)
	wget --no-use-server-timestamps $(COMPONENT_ZIPBALL_URL) -O $(COMPONENT_ZIPBALL_PATH)

