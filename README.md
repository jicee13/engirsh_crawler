# engrish_crawler
Getting my feet wet with Rust so I made a little crawler that scrapes [engrish.com](http://www.engrish.com/) and pulls all the images I want and dumps the links in a database. One day I will make an app to accompany this. One day.

## To Run
Make sure you have a [firebase](https://console.firebase.google.com) account set up and the write access to the realtime database is public (haven't bothered with auth anything yet). Then just sub in the database URL in the code.

`cargo build`

`cargo run`
