# IcarusDownloadManager

IcarusDownloadManager is a CLI program tool that has the feature of uploading and downloading songs from the [Icarus](https://github.com/amazing-username/Icarus) Music Server.

### Prerequisites

* CMake
* gcc
* libcurl
* [cpr](http://whoshuu.github.io/cpr/)

### Installing

Clone the repository and esnure that the cpr c++ module is implemented by checking the contents of the cpr directory. If you notice a directory structure and a *CMakeList.txt* file then you are fine. Otherwise implement the modules with the following command:


```
git submodule update --init --recursive
```

Once that is complete, verify the contents of the cpr directory and there should be a *CMakeList.txt* file. Now you must compile and link the project

```
cmake .
make
```

The program has been built and can be executed by the binary file *icd*


## Built With

* c++

## Contributing

Please read [CONTRIBUTING.md](CONTRIBUTING.md) for details on the code of conduct, and the process for submitting pull requests to the project.

## Versioning

No version has been released 

## Authors

* **Kun Deng** - [amazing-username](https://github.com/amazing-username)

See also the list of [contributors](https://github.com/amazing-username/Icarus/graphs/contributors) who participated in this project.

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details

