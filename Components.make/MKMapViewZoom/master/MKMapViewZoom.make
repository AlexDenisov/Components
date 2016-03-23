NAME=MKMapViewZoom
VERSION=master
GH_REPO=johndpope/MKMapViewZoom

### URLs

COMPONENT_URL=https://raw.githubusercontent.com/johndpope/MKMapViewZoom/master

### Paths

COMPONENTS_BUILD_CACHE_PATH ?= $(HOME)/Library/Caches/Components
COMPONENTS_INSTALL_PATH ?= ./Components

COMPONENT_BUILD_PATH=$(COMPONENTS_BUILD_CACHE_PATH)/$(NAME)
COMPONENT_SOURCE_PATH=$(COMPONENT_BUILD_PATH)/$(NAME)-$(VERSION)

COMPONENT_INSTALL_PATH=$(COMPONENTS_INSTALL_PATH)/$(NAME)

### Targets

.PHONY: install update uninstall clean prepare purge

install: $(COMPONENT_INSTALL_PATH)

uninstall:
	rm -rf $(COMPONENT_INSTALL_PATH)

update: uninstall install

clean:
	rm -rf $(COMPONENT_SOURCE_PATH)

purge: uninstall clean

### Artefacts

$(COMPONENT_INSTALL_PATH): $(COMPONENT_SOURCE_PATH)
	mkdir -p $(COMPONENT_INSTALL_PATH)

	cp -Rv $(COMPONENT_SOURCE_PATH)/MKMapView+ZoomLevel.[hm] $(COMPONENT_INSTALL_PATH)

$(COMPONENT_SOURCE_PATH):
	mkdir -p $(COMPONENT_BUILD_PATH)
	mkdir -p $(COMPONENT_SOURCE_PATH)

	wget --no-use-server-timestamps $(COMPONENT_URL)/MKMapView+ZoomLevel.h -O $(COMPONENT_SOURCE_PATH)/MKMapView+ZoomLevel.h
	wget --no-use-server-timestamps $(COMPONENT_URL)/MKMapView+ZoomLevel.m -O $(COMPONENT_SOURCE_PATH)/MKMapView+ZoomLevel.m

