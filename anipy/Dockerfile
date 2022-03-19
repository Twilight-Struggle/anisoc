FROM pytorch/pytorch:latest

RUN apt-get update && apt-get clean \
&&  rm -rf /var/lib/apt/lists/*

RUN python3 -m pip install --upgrade pip \
&&  pip install --no-cache-dir \
    black \
    grpcio \
    grpcio-tools

WORKDIR /workspace
COPY anipy.py .
COPY animalai_pb2_grpc.py .
COPY animalai_pb2.py .

ENTRYPOINT ["python3", "anipy.py"]

# modelが1GBくらいあるので./modelにmodelを入れてマウントしたほうが手軽
# docker run --name anipy_ins -v (pwd)/model:/model -p 8888:8888 anipy