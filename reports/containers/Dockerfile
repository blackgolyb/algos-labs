FROM texlive/texlive:latest

RUN echo "deb http://deb.debian.org/debian/ testing contrib" >> /etc/apt/sources.list && \
    apt-get update && \
    apt-get install -y \
    msttcorefonts \
    fontconfig \
    --no-install-recommends && \
    fc-cache -f -v && \
    apt-get clean && rm -rf /var/lib/apt/lists/*
    
WORKDIR /docs
CMD ["bash"]
