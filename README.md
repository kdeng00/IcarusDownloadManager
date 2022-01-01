# IcarusDownloadManager

IcarusDownloadManager is a Linux CLI software client application that has the feature of uploading and downloading songs from the [Icarus](https://github.com/kdeng00/Icarus) Music Server. 


## Built With

* C++ with C++17 features
* CMake
* GCC >= 9
* [conan](https://github.com/conan-io/conan)
* [json](https://github.com/nlohmann/json)
* [openssl](https://github.com/openssl/openssl)
* [curl](https://github.com/curl/curl)
* [cpr](https://github.com/libcpr/cpr)


### Getting Started

Build the project:

```
git clone --recursive https://github.com/kdeng00/IcarusDownloadManager


mkdir build
cd build

conan install .. --build

cmake -DCMAKE_BUILD_TYPE=RELEASE
cmake --build . -j
```

The program has been built and can be executed by the binary file *icd*. For information on how to use icd, merely execute the program without any command line arguments.

### Downloading Song
``icd download -u spacecadet -p stellar40 -h https://icarus.com -b 15``

### Uploading Song
``icd upload -u spacecadet -p stellar40 -h https://icarus.com -s /path/of/song.mp3``

### Uploading Song with metadata

```BASH
icd upload-meta -u spacecadet -p stellar40 -h https://icarus.com -s /path/of/song.mp3 -t 1 -m /path/to/metadata/config/collection.json -ca /path/to/cover/art/image.png
```

### Uploading Song with metadata from directory

```BASH
icd upload-meta -u spacecadet -p stellar40 -h https://icarus.com -smca /path/where/songs/and/metadata/exist
```


### Retrieving Song in json
``icd retrieve -u spacecadet -p stellar40 -h https://icarus.com -rt songs``

### Deleting Song
``icd delete -u spacecadet -p stellar40 -h https://icarus.com -D 15``


## Contributing

Please read [CONTRIBUTING.md](CONTRIBUTING.md) for details on the code of conduct, and the process for submitting pull requests to the project.

## Versioning

[v0.1.1](https://github.com/kdeng00/IcarusDownloadManager/releases/tag/v0.1.1)  
[v0.1.0](https://github.com/kdeng00/IcarusDownloadManager/releases/tag/0.1.0)

## Authors

* **Kun Deng** - [kdeng00](https://github.com/kdeng00)

See also the list of [contributors](https://github.com/kdeng00/Icarus/graphs/contributors) who participated in this project.

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details
