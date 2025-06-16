# Stage 1: Build OpenCV
FROM ubuntu:22.04 AS build

ENV DEBIAN_FRONTEND=noninteractive
ENV TZ=UTC

RUN apt-get update --allow-insecure-repositories && \
    apt-get install -y --no-install-recommends ca-certificates && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*


RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    build-essential \
    cmake \
    git \
    unzip \
    wget \
    clang \ 
    libclang-dev \
    wget \
    pkg-config \
    curl && \
    rm -rf /var/lib/apt/lists/*

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

#RUN wget -O opencv.zip https://github.com/opencv/opencv/archive/${OPENCV_VERSION}.zip && \
#    unzip -q opencv.zip && \
#    mv opencv-${OPENCV_VERSION} opencv
# Download and extract OpenCV and opencv_contrib 4.8.1
RUN wget -O opencv.zip https://github.com/opencv/opencv/archive/4.8.1.zip
RUN wget -O opencv_contrib.zip https://github.com/opencv/opencv_contrib/archive/4.8.1.zip
RUN unzip opencv.zip && rm opencv.zip 
RUN unzip opencv_contrib.zip && rm opencv_contrib.zip


RUN wget https://ffmpeg.org/releases/ffmpeg-4.4.4.tar.gz && \
    tar -xzf ffmpeg-4.4.4.tar.gz && \
    mv ffmpeg-4.4.4 /ffmpeg

ENV PKG_CONFIG_PATH="/usr/lib/x86_64-linux-gnu/pkgconfig:${PKG_CONFIG_PATH}"
ENV PKG_CONFIG_PATH="/usr/lib/pkgconfig/${PKG_CONFIG_PATH}"


RUN apt-get update && apt-get install -y --no-install-recommends \
    libx264-dev \
    libx265-dev \
    libvpx-dev \
    libfdk-aac-dev \
    libmp3lame-dev \
    libopus-dev \
    libaom-dev 

WORKDIR /ffmpeg
RUN ./configure \
    --prefix=/opt/ffmpeg_static \
    --disable-x86asm \
    --enable-static \
    --disable-shared \
    --enable-gpl \
    --enable-libx264 \
    --enable-libx265 


RUN make -j$(nproc) && make install

# add the libraries here and also in the include path 
#RUN apt-get update && apt-get install -y --no-install-recommends \
      #libpng-dev \
      #zlib1g-dev \

      #libavcodec-dev \
      #libavformat-dev \
      #libswscale-dev \
      #libavutil-dev 
      
      #libavcodec-dev \ 
      #libavformat-dev \
      #libswscale-dev 

ENV PKG_CONFIG_PATH="/opt/ffmpeg_static/lib/pkgconfig:${PKG_CONFIG_PATH}"
ENV LD_LIBRARY_PATH="/opt/ffmpeg_static/lib:${LD_LIBRARY_PATH}"
ENV PKG_CONFIG_LIBDIR="/opt/ffmpeg_static/lib/:${PKG_CONFIG_LIBDIR}"




# Build and install OpenCV statically to /opt/opencv
RUN mkdir -p /opencv-4.8.1/build && cd /opencv-4.8.1/build && \
    cmake -DCMAKE_BUILD_TYPE=Release \
          -D BUILD_CUDA_STUBS=OFF \
	        -D BUILD_DOCS=OFF \
	        -D BUILD_EXAMPLES=OFF \
	        -D BUILD_IPP_IW=ON \
	        -D BUILD_ITT=ON \ \
	        -D BUILD_JASPER=OFF \
	        -D BUILD_JAVA=OFF \
	        -D BUILD_JPEG=ON \
	        -D BUILD_OPENEXR=ON \
	        -D BUILD_OPENJPEG=ON \
	        -D WITH_OPENJPEG=OFF \
	        -D BUILD_PERF_TESTS=OFF \
	        -D BUILD_PNG=ON \
	        -D WITH_PNG=ON \
	        -D BUILD_PROTOBUF=ON \
	        -D BUILD_TBB=ON \
	        -D BUILD_TESTS=OFF \
	        -D BUILD_WEBP=ON \
	        -D BUILD_WITH_DEBUG_INFO=OFF \
	        -D BUILD_WITH_DYNAMIC_IPP=OFF \
	        -D BUILD_ZLIB=ON \
          -D WITH_ZLIB=ON \
	        -D BUILD_opencv_apps=OFF \
	        -D BUILD_opencv_python2=OFF \
	        -D BUILD_opencv_python3=OFF \
	        -D CMAKE_INSTALL_PREFIX=/opt/opencv \
	        -D CV_DISABLE_OPTIMIZATION=OFF \
	        -D CV_ENABLE_INTRINSICS=ON \
	        -D ENABLE_CONFIG_VERIFICATION=OFF \
	        -D ENABLE_FAST_MATH=OFF \
	        -D ENABLE_LTO=OFF \
	        -D ENABLE_PIC=ON \
	        -D ENABLE_PRECOMPILED_HEADERS=OFF \
	        -D INSTALL_CREATE_DISTRIB=OFF \
	        -D INSTALL_C_EXAMPLES=OFF \
	        -D INSTALL_PYTHON_EXAMPLES=OFF \
	        -D INSTALL_TESTS=OFF \
	        -D OPENCV_ENABLE_MEMALIGN=OFF \
	        -D OPENCV_ENABLE_NONFREE=ON \
	        -D OPENCV_GENERATE_PKGCONFIG=OFF \
	        -D PROTOBUF_UPDATE_FILES=OFF \
	        -D WITH_ADE=ON \
	        -D WITH_ARAVIS=OFF \
	        -D WITH_CLP=OFF \
	        -D WITH_CUBLAS=OFF \
	        -D WITH_CUDA=OFF \
	        -D WITH_CUFFT=OFF \
	        -D WITH_EIGEN=ON \
	        -D WITH_GDCM=OFF \ \
	        -D WITH_GIGEAPI=OFF \
	        -D WITH_GSTREAMER_0_10=OFF \
	        -D WITH_GTK=OFF \
	        -D WITH_GTK_2_X=OFF \
	        -D WITH_HALIDE=OFF \
	        -D WITH_IMGCODEC_HDcR=ON \
	        -D WITH_IMGCODEC_PXM=ON \
	        -D WITH_IMGCODEC_SUNRASTER=ON \
	        -D WITH_INF_ENGINE=OFF \
	        -D WITH_IPP=OFF \
	        -D WITH_ITT=OFF \
	        -D WITH_JASPER=OFF \
	        -D WITH_JPEG=OFF \
	        -D WITH_LIBV4L=OFF \
	        -D WITH_MATLAB=OFF \
	        -D WITH_MFX=OFF \
	        -D WITH_OPENCL=OFF \
	        -D WITH_OPENCLAMDBLAS=OFF \
	        -D WITH_OPENCLAMDFFT=OFF \
	        -D WITH_OPENCL_SVM=OFF \
	        -D WITH_OPENEXR=OFF \
	        -D WITH_OPENMP=OFF \
	        -D WITH_OPENNI2=OFF \
	        -D WITH_OPENNI=OFF \
	        -D WITH_OPENVX=OFF \
	        -D WITH_PROTOBUF=ON \
	        -D WITH_PTHREADS_PF=ON \
	        -D WITH_PVAPI=OFF \
	        -D WITH_QUIRC=ON \
	        -D WITH_TBB=OFF \
	        -D WITH_TIFF=ON \
	        -D WITH_UNICAP=OFF \
	        -D WITH_V4L=ON \
	        -D WITH_VA=ON \
	        -D WITH_VA_INTEL=ON \
	        -D WITH_VTK=ON \
	        -D WITH_WEBP=OFF \
	        -D WITH_XIMEA=OFF \
	        -D WITH_XINE=OFF \
	        -D WITH_XIMEA=OFF \
	        -D WITH_XINE=OFF \
          -D BUILD_SHARED_LIBS=OFF \
          -D BUILD_TIFF=ON \
          -D BUILD_opencv_freetype=OFF \
          -D OPENCV_FORCE_3RDPARTY_BUILD=ON \
          -D WITH_1394=OFF \
          -D WITH_FFMPEG=OFF \
          -D OPENCV_FFMPEG_SKIP_BUILD_CHECK=ON \
          -D WITH_FREETYPE=OFF \
          -D WITH_GDAL=OFF \
          -D WITH_GPHOTO2=OFF \
          -D WITH_GSTREAMER=OFF\
          -D WITH_GTK=OFF \
          -D WITH_LAPACK=OFF \
          -D WITH_OPENGL=OFF \
          -D OPENCV_GENERATE_PKGCONFIG=ON \
          -D WITH_QT=OFF \
          -D OPENCV_EXTRA_MODULES_PATH=/opencv_contrib-4.8.1/modules \
          .. && \
    cmake --build . --target install --config Release --parallel 8

#ENV OPENCV_LINK_LIBS="opencv_objdetect,opencv_videoio,opencv_imgcodecs,opencv_imgproc,opencv_core,libippiw,libittnotify,libippicv,z"
ENV OPENCV_LINK_LIBS="static=opencv_objdetect,static=opencv_videoio,static=opencv_imgcodecs,static=opencv_imgproc,static=opencv_core,libavcodec,libavdevice,libavfilter,libavformat,libavutil,libswresample,libswscale,zlib,liblibpng,liblibtiff"
ENV OPENCV_LINK_PATHS="/opt/opencv/lib,/opt/opencv/lib/opencv4/3rdparty/,/opt/ffmpeg_static/lib/,/usr/lib/x86_64-linux-gnu/"
ENV OPENCV_INCLUDE_PATHS=/opt/opencv/include,/opt/opencv/include/opencv4,/usr/lib/x86_64-linux-gnu/
ENV PKG_CONFIG_PATH="/opt/opencv/lib/pkgconfig:${PKG_CONFIG_PATH}"

# Add Rust cargo to PATH for the build process
# find / -type f -name cargo 2>/dev/null
#


WORKDIR /face-cropper
COPY . .  


ENV PATH="/root/.cargo/bin:${PATH}"

RUN cargo build --release --verbose

#RUN ldd /face-cropper/target/release/face-cropper

FROM scratch
COPY --from=build /face-cropper/target/release/face-cropper / 
ENTRYPOINT ["/face-cropper"]
