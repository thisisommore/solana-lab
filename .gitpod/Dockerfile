FROM gitpod/workspace-full
USER gitpod
RUN sh -c "$(curl -sSfL https://release.solana.com/v1.15.2/install)"
ENV PATH="/home/gitpod/.local/share/solana/install/active_release/bin:$PATH"
RUN solana config set --url devnet
RUN npm install -g mocha @coral-xyz/anchor-cli
