export DOCKER_DEFAULT_PLATFORM=linux/amd64
PROJECT_DIR := $(realpath $(dir $(lastword $(MAKEFILE_LIST))))

export SRC_FOLDER=${PROJECT_DIR}/src
export BUILD_FOLDER=${PROJECT_DIR}/build
export MAIN_FILE=main.tex
export LATEX_PROGRAM=xelatex
export LATEX_ARGS=

all: build

build:
	docker compose -f ${PROJECT_DIR}/containers/docker-compose.yml up

rebuild:
	docker compose -f ${PROJECT_DIR}/containers/docker-compose.yml up --build

build_locale:
	mkdir -p ${BUILD_FOLDER} && \
	cd src \
	&& ${LATEX_PROGRAM} ${LATEX_ARGS} -output-directory=${BUILD_FOLDER} ${MAIN_FILE}\
	&& cd ../


.PHONY: all build rebuild build_locale