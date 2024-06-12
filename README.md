# IcarusDownloadManager

IcarusDownloadManager is a CLI software client application that has the feature of uploading and downloading songs from the [Icarus](https://github.com/kdeng00/Icarus) Music Server. 


## Built With

* Rust
* Cargo
* futures
* http
* reqwst
* serde
* serde_json
* tokio
* tokio-util


### Getting Started

Clone the repo

```BASH
git clone --recursive https://github.com/kdeng00/IcarusDownloadManager
```


Build the project:

```BASH
cd IcarusDownloadManager
cargo build
```

The program has been built and can be executed by the binary file *icarus-dm*. For information on how to use icarua-dm, merely execute the program without any command line arguments.

### Downloading Song

```BASH
icarus-dm download -u spacecadet -p stellar40 -h https://icarus.com -b 15
```

### Uploading Song with metadata

```BASH
icarus-dm upload-meta -u spacecadet -p stellar40 -h https://icarus.com -s /path/of/song.mp3 -t 1 -m /path/to/metadata/config/collection.json -ca /path/to/cover/art/image.png
```

### Uploading Song with metadata from directory

```BASH
icarus-dm upload-meta -u spacecadet -p stellar40 -h https://icarus.com -smca /path/where/songs/and/metadata/exists/
```

### Retrieving Song in json

```Bash
icarus-dm retrieve -u spacecadet -p stellar40 -h https://icarus.com -rt songs
```

### Deleting Song

```BASH
icarus-dm delete -u spacecadet -p stellar40 -h https://icarus.com -D 15
```


## Contributing

Please read [CONTRIBUTING.md](CONTRIBUTING.md) for details on the code of conduct, and the process for submitting pull requests to the project.

## Versioning

[v0.3.2](https://github.com/kdeng00/IcarusDownloadManager/releases/tag/v0.3.2)  
[v0.3.0](https://github.com/kdeng00/IcarusDownloadManager/releases/tag/v0.3.0)  
[v0.2.0](https://github.com/kdeng00/IcarusDownloadManager/releases/tag/v0.2.0)  
[v0.1.2](https://github.com/kdeng00/IcarusDownloadManager/releases/tag/v0.1.2)  
[v0.1.1](https://github.com/kdeng00/IcarusDownloadManager/releases/tag/v0.1.1)  
[v0.1.0](https://github.com/kdeng00/IcarusDownloadManager/releases/tag/0.1.0)

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details
