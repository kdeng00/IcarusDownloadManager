# IcarusDownloadManager

IcarusDownloadManager is a Linux CLI software client application that has the feature of uploading and downloading songs from the [Icarus](https://github.com/amazing-username/Icarus) Music Server. 


## Built With

* C++
* CMake
* GCC
* [Hunter](https://github.com/ruslo/hunter)
* libCurl
* [json](https://github.com/nlohmann/json)
* [cpr](http://whoshuu.github.io/cpr/)


### Getting Started

Build the project:

```
export HUNTER_ROOT=/path/to/download/hunter/files/for/dependencies
mkdir _build
cd _build
cmake -H. -B_builds -DHUNTER_STATUS_DEBUG=ON -DCMAKE_BUILD_TYPE=DEBUG
cmake --build _builds --config Debug
make
```

The program has been built and can be executed by the binary file *icd*. For information on how to use icd, merely execute the program without any command line arguments.

### Downloading Song
``icd download -u spacecadet -p stellar40 -h https://icarus.com -b 15``

### Uploading Song
``icd upload -u spacecadet -p stellar40 -h https://icarus.com -s /path/of/song.mp3``

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

* **Kun Deng** - [amazing-username](https://github.com/amazing-username)

See also the list of [contributors](https://github.com/amazing-username/Icarus/graphs/contributors) who participated in this project.

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details
