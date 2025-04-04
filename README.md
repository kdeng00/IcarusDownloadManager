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
* icarus_models


### Getting Started

Clone the repo

```BASH
git clone git@github.com:kdeng00/IcarusDownloadManager.git
```


Build the project:

```BASH
cd IcarusDownloadManager
cargo build
```

Even though this project is open source, there are some libraries that are closed source (may be opened later).
In order to successfully build it, your ssh public key would be needed to add to the closed libraries. If you
have interest, something could be worked out to provide access.

The program has been built and can be executed by the binary file *icarus-dm*. For information on how to use icarua-dm, merely execute the program without any command line arguments.

### Downloading Song

```BASH
icarus-dm download -u spacecadet -p stellar40 -h https://icarus.com -b 15
```

### Uploading Song with metadata

```BASH
icarus-dm upload-meta -u spacecadet -p stellar40 -h https://icarus.com -s /path/of/song.flac -t 1 -m /path/to/metadata/config/collection.json -ca /path/to/cover/art/image.png
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



## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details
