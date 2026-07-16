# sidm

sidm (SoarIcarusDownloadManager) is a CLI software client application that has the feature of uploading and downloading songs from the [soaricarus_api](https://git.kundeng.us/phoenix/soaricarus_api) Music Server. 



### Getting Started

Clone the repo

```BASH
git clone git@git.kundeng.us:phoenix/sidm.git
```


Build the project:

```BASH
cd sidm
cargo build
```


### Downloading Song

```BASH
sidm download -u spacecadet -p stellar40 -h https://icarus.com -ha https://auth.icarus.com -b e8407fc6-edd2-44c1-993f-08dd7324d91a
```

### Uploading Song with metadata

```BASH
sidm upload-meta -u spacecadet -p stellar40 -h https://icarus.com -ha https://auth.icarus.com -s /path/of/song.flac -t 1 -m /path/to/metadata/config/collection.json -ca /path/to/cover/art/image.png
```

### Uploading Song with metadata from directory

```BASH
sidm upload-meta -u spacecadet -p stellar40 -h https://icarus.com -ha https://auth.icarus.com -smca /path/where/songs/and/metadata/exists/
```

### Retrieving Song in json

```Bash
sidm retrieve -u spacecadet -p stellar40 -h https://icarus.com -ha https://auth.icarus.com -rt songs
```

### Deleting Song

```BASH
sidm delete -u spacecadet -p stellar40 -h https://icarus.com -ha https://auth.icarus.com -D e8407fc6-edd2-44c1-993f-08dd7324d91a
```


## Contributing

Please read [CONTRIBUTING.md](CONTRIBUTING.md) for details on the code of conduct, and the process for submitting pull requests to the project.



## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details
