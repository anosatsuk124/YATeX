# See here for image contents: https://github.com/microsoft/vscode-dev-containers/tree/v0.241.1/containers/rust/.devcontainer/base.Dockerfile

# [Choice] Debian OS version (use bullseye on local arm64/Apple Silicon): buster, bullseye
ARG VARIANT="bullseye"
FROM mcr.microsoft.com/vscode/devcontainers/rust:0-${VARIANT}

# [Optional] Uncomment this section to install additional packages.
# RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
#     && apt-get -y install --no-install-recommends <your-package-list-here>

RUN apt-get update && apt-get install -y wget tar gzip perl git libyaml-tiny-perl libfile-homedir-perl libunicode-linebreak-perl
# icu-data-full
WORKDIR /tmp

RUN wget http://mirror.ctan.org/systems/texlive/tlnet/install-tl-unx.tar.gz

RUN tar xvf install-tl-unx.tar.gz

RUN cd install-tl-2* && \
    perl ./install-tl -no-gui -repository http://mirror.ctan.org/systems/texlive/tlnet/ --no-interaction --scheme small

RUN /usr/local/texlive/2022/bin/x86_64-linux/tlmgr path add

RUN tlmgr init-usertree

RUN tlmgr update --self --all

RUN tlmgr install luatexja
RUN tlmgr install latexmk
RUN tlmgr install latex-bin
RUN tlmgr install cjk
RUN tlmgr install latexmk
RUN tlmgr install haranoaji haranoaji-extra

RUN wget https://github.com/jgm/pandoc/releases/download/2.18/pandoc-2.18-linux-amd64.tar.gz && \
    tar xvf pandoc-2.18-linux-amd64.tar.gz && \
    mv pandoc-2.18 /usr/local/pandoc && \
    echo "export PATH=/usr/local/pandoc/bin:${PATH}" > /etc/profile.d/pandoc.sh

RUN tlmgr install latexindent
RUN tlmgr path add