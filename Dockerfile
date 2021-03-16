FROM rust:1.50

ENV USER=rust
ENV HOME /${USER}/'rust-playground'

WORKDIR ${HOME}
RUN mkdir tmpA && cd tmpA
COPY . ${HOME}/tmpA
COPY /build-resources/hidden .

RUN cd ~/tmpA argo install --path .
RUN rm -rf ~/tmpA

RUN apt-get update && apt-get install vim exuberant-ctags -y && \ 
    apt-get autoremove
#RUN apt-get autoremove

# Set user and group
ARG uid=1000
ARG gid=1000
RUN groupadd -g ${gid} ${USER}
RUN useradd -u ${uid} -g ${USER} -s /bin/sh -m ${USER} # <--- the '-m' create a user home directory

USER ${USER}
CMD tail -f /dev/null
