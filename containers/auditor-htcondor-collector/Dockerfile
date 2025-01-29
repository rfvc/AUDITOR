#################
# BUILDER IMAGE #
#################

FROM htcondor/submit AS builder

# install build dependencies and create python env
RUN set -ex                                    \
    && yum -y update                           \
    && yum -y groupinstall 'Development Tools' \
    && python3 -m venv /env                    \
    && /env/bin/pip install --upgrade pip      \
    && yum clean all

# install rust toolchain
SHELL ["/bin/bash", "-o", "pipefail", "-c"]
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# activate python env
ENV VIRTUAL_ENV=/env
ENV PATH=/env/bin:$PATH

# copy code
COPY . /code

# build pyauditor wheel
WORKDIR /code/pyauditor
RUN pip wheel .

# install python package
WORKDIR /code/collectors/htcondor
RUN    pip install --no-cache-dir /code/pyauditor/*.whl \
    && pip install --no-cache-dir .


################
# RUNNER IMAGE #
################

FROM htcondor/submit AS runner

# prevent python from writing *.pyc files to disc
ENV PYTHONDONTWRITEBYTECODE=1
# prevent python from buffering stdout and stderr
ENV PYTHONUNBUFFERED=1

# update all
RUN set -ex && yum -y update && yum clean all

# copy python env from builder image
COPY --from=builder /env /env

# activate python env
ENV VIRTUAL_ENV=/env
ENV PATH=/env/bin:$PATH

WORKDIR /app
ENTRYPOINT ["auditor-htcondor-collector"]
