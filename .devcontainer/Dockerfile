FROM mcr.microsoft.com/vscode/devcontainers/rust

ARG INSTALL_ZSH="true"
ARG UPGRADE_PACKAGES="true"
ARG USERNAME=node
ARG USER_UID=1001
ARG USER_GID=$USER_UID

ENV NVM_DIR=/usr/local/share/nvm
ENV NVM_SYMLINK_CURRENT=true \ 
    PATH=${NVM_DIR}/current/bin:${PATH}

COPY library-scripts/*.sh /tmp/library-scripts/
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    # Remove imagemagick due to https://security-tracker.debian.org/tracker/CVE-2019-10131
    && apt-get purge -y imagemagick imagemagick-6-common \
    # Install common packages, non-root user
    && /bin/bash /tmp/library-scripts/common-debian.sh "${INSTALL_ZSH}" "${USERNAME}" "${USER_UID}" "${USER_GID}" "${UPGRADE_PACKAGES}" \
    # Update yarn and install nvm
    && rm -rf /opt/yarn-* /usr/local/bin/yarn /usr/local/bin/yarnpkg \
    && /bin/bash /tmp/library-scripts/node-debian.sh "${NVM_DIR}" "12" "${USERNAME}" \
    # Clean up
    && apt-get autoremove -y && apt-get clean -y && rm -rf /var/lib/apt/lists/* /root/.gnupg /tmp/library-scripts

# Configure global npm install location
# ARG NPM_GLOBAL=/usr/local/share/npm-global
# ENV PATH=${PATH}:${NPM_GLOBAL}/bin
# RUN mkdir -p ${NPM_GLOBAL} \
#     && chown ${USERNAME}:root ${NPM_GLOBAL} \
#     && npm config -g set prefix ${NPM_GLOBAL} \
#     && sudo -u ${USERNAME} npm config -g set prefix ${NPM_GLOBAL} \
#     && echo "if [ \"\$(stat -c '%U' ${NPM_GLOBAL})\" != \"${USERNAME}\" ]; then sudo chown -R ${USER_UID}:root ${NPM_GLOBAL} ${NVM_DIR}; fi" \
#     | tee -a /root/.bashrc /root/.zshrc /home/${USERNAME}/.bashrc >> /home/${USERNAME}/.zshrc

# Install eslint globally
# RUN sudo -u ${USERNAME} npm install -g eslint

# Install ts tslint globally
# RUN sudo -u ${USERNAME} npm install -g tslint typescript

# Install User Enviroment
ENV DEBIAN_FRONTEND=noninteractive
# if you want to change
ENV TZ=Asia/Seoul
RUN apt-get update \
    && apt-get -y install --no-install-recommends git iproute2 procps zsh python3-pip software-properties-common netcat locales locales-all \
    # locale
    && sed -i -e 's/# en_US.UTF-8 UTF-8/en_US.UTF-8 UTF-8/' /etc/locale.gen \
    && locale-gen \
    #
    # set timezone
    && ln -snf /usr/share/zoneinfo/$TZ /etc/localtime && echo $TZ > /etc/timezone \
    #
    # install rust toolchain nightly
    # && rustup toolchain install nightly \
    # && rustup target add wasm32-unknown-unknown --toolchain nightly \
    #
    # install Nearup
    # && curl --proto '=https' --tlsv1.2 -sSfL https://up.near.dev | python3 \
    #
    # Clean up
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*
    #
    # Install Dependencies for NearD 
    # && apt-get update \
    # && apt-get -y install git binutils-dev libcurl4-openssl-dev zlib1g-dev libdw-dev libiberty-dev cmake gcc g++ python protobuf-compiler libssl-dev pkg-config clang llvm \
    # && git clone https://github.com/nearprotocol/nearcore.git \
    # && cd nearcore \
    # && git checkout 1.11.1 \
    # && make release
ENV LANG en_US.UTF-8
ENV LANGUAGE en_US:en
ENV LC_ALL en_US.UTF-8
ENV DEBIAN_FRONTEND=dialog