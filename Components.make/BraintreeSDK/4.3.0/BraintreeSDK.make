NAME=BraintreeSDK
VERSION=4.3.0

GH_REPO=braintree/braintree_ios

COMPONENTS_BUILD_CACHE_PATH ?= $(HOME)/Library/Caches/Components
COMPONENTS_INSTALL_PATH ?= ./Components

COMPONENT_BUILD_PATH=$(COMPONENTS_BUILD_CACHE_PATH)/$(NAME)
COMPONENT_SOURCE_PATH=$(COMPONENT_BUILD_PATH)/$(NAME)-$(VERSION)
COMPONENT_PROJECT_PATH=$(COMPONENT_SOURCE_PATH)/braintree_ios-$(VERSION)

COMPONENT_INSTALL_PATH=$(COMPONENTS_INSTALL_PATH)/$(NAME)/

COMPONENT_ZIPBALL_PATH=$(COMPONENT_BUILD_PATH)/$(NAME)-$(VERSION).zip

BUILD_DIR=$(COMPONENT_PROJECT_PATH)/Build
BUILD_DIR_SIMULATOR=$(BUILD_DIR)/Release-iphonesimulator
BUILD_DIR_IPHONE=$(BUILD_DIR)/Release-iphoneos

COMPONENT_ARTEFACT_PATH=$(BUILD_DIR)/Release-universal

### URLs
COMPONENT_ZIPBALL_URL=https://github.com/$(GH_REPO)/archive/$(VERSION).zip

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

$(COMPONENT_INSTALL_PATH): $(COMPONENT_ARTEFACT_PATH)
	mkdir -p $(COMPONENT_INSTALL_PATH)
	cp -rv $(COMPONENT_ARTEFACT_PATH)/* $(COMPONENT_INSTALL_PATH)

$(COMPONENT_ARTEFACT_PATH): $(COMPONENT_SOURCE_PATH)
	cd $(COMPONENT_SOURCE_PATH)/braintree_ios-$(VERSION) && xcodebuild -project Braintree.xcodeproj \
             -scheme Braintree \
             -IDEBuildOperationMaxNumberOfConcurrentCompileTasks=`sysctl -n hw.ncpu` \
             -sdk iphonesimulator \
             -destination 'platform=iOS Simulator,name=iPhone 6S Plus,OS=latest' \
             -configuration Release \
             CONFIGURATION_BUILD_DIR=$(BUILD_DIR_SIMULATOR) \
             clean build 

	cd $(COMPONENT_SOURCE_PATH)/braintree_ios-$(VERSION) && xcodebuild -project Braintree.xcodeproj \
             -scheme Braintree \
             -IDEBuildOperationMaxNumberOfConcurrentCompileTasks=`sysctl -n hw.ncpu` \
             -sdk iphoneos \
             -configuration Release \
             CONFIGURATION_BUILD_DIR=$(BUILD_DIR_IPHONE) \
             clean build 

	mkdir -p $(COMPONENT_ARTEFACT_PATH)
	cp -Rv $(BUILD_DIR_IPHONE)/* $(COMPONENT_ARTEFACT_PATH)/

	lipo $(BUILD_DIR_SIMULATOR)/libBraintree.a $(BUILD_DIR_IPHONE)/libBraintree.a -create -output $(COMPONENT_ARTEFACT_PATH)/libBraintree.a
	lipo $(BUILD_DIR_SIMULATOR)/libPayPalDataCollector-StaticLibrary.a $(BUILD_DIR_IPHONE)/libPayPalDataCollector-StaticLibrary.a -create -output $(COMPONENT_ARTEFACT_PATH)/libPayPalDataCollector-StaticLibrary.a
	lipo $(BUILD_DIR_SIMULATOR)/libPayPalOneTouch-StaticLibrary.a $(BUILD_DIR_IPHONE)/libPayPalOneTouch-StaticLibrary.a -create -output $(COMPONENT_ARTEFACT_PATH)/libPayPalOneTouch-StaticLibrary.a

	cp -rv $(COMPONENT_PROJECT_PATH)/Braintree3DSecure/Public/*.h $(COMPONENT_ARTEFACT_PATH)/include
	cp -rv $(COMPONENT_PROJECT_PATH)/BraintreeApplePay/Public/*.h $(COMPONENT_ARTEFACT_PATH)/include
	cp -rv $(COMPONENT_PROJECT_PATH)/BraintreeCard/Public/*.h $(COMPONENT_ARTEFACT_PATH)/include
	cp -rv $(COMPONENT_PROJECT_PATH)/BraintreeCore/Public/*.h $(COMPONENT_ARTEFACT_PATH)/include
	cp -rv $(COMPONENT_PROJECT_PATH)/BraintreeDataCollector/Public/*.h $(COMPONENT_ARTEFACT_PATH)/include
	cp -rv $(COMPONENT_PROJECT_PATH)/BraintreePayPal/Public/*.h $(COMPONENT_ARTEFACT_PATH)/include
	cp -rv $(COMPONENT_PROJECT_PATH)/BraintreeUI/Public/*.h $(COMPONENT_ARTEFACT_PATH)/include
	cp -rv $(COMPONENT_PROJECT_PATH)/BraintreeUnionPay/Public/*.h $(COMPONENT_ARTEFACT_PATH)/include
	cp -rv $(COMPONENT_PROJECT_PATH)/BraintreeVenmo/Public/*.h $(COMPONENT_ARTEFACT_PATH)/include

$(COMPONENT_SOURCE_PATH): $(COMPONENT_ZIPBALL_PATH)
	unzip $(COMPONENT_ZIPBALL_PATH) -d $(COMPONENT_SOURCE_PATH)

$(COMPONENT_ZIPBALL_PATH):
	mkdir -p $(COMPONENT_BUILD_PATH)
	wget --no-use-server-timestamps $(COMPONENT_ZIPBALL_URL) -O $(COMPONENT_ZIPBALL_PATH)

