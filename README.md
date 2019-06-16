# IcarusDownloadManager

IcarusDownloadManager is a Linux UI software client application that has the feature of uploading and downloading songs from the [Icarus](https://github.com/amazing-username/Icarus) Music Server. 


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

The program has been built and can be executed by the binary file *icd*


## Built With

* C++
* CMake
* GCC
* [Hunter](https://github.com/ruslo/hunter)
* libCurl
* [json](https://github.com/nlohmann/json)
* [cpr](http://whoshuu.github.io/cpr/)

## Contributing

Please read [CONTRIBUTING.md](CONTRIBUTING.md) for details on the code of conduct, and the process for submitting pull requests to the project.

## Versioning

[v0.1.0](https://github.com/amazing-username/IcarusDownloadManager/releases/tag/0.1.0)

## Authors

* **Kun Deng** - [amazing-username](https://github.com/amazing-username)

See also the list of [contributors](https://github.com/amazing-username/Icarus/graphs/contributors) who participated in this project.

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details
