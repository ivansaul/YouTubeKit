# rstube

Rust client for the public YouTube / YouTube Music API (Innertube), inspired by [Rustypipe](https://codeberg.org/ThetaDev/rustypipe) and [pytubefix](https://github.com/JuanBindez/pytubefix).

> [!WARNING]
> This is a work in progress and may not be fully functional yet.

## Usage

Below are examples of how to use the main functionalities of `rstube`.

### Searching for Videos

You can search for videos using the `search` method. You can specify the language and location to refine the results.

```rust
use rstube::RusTube;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = RusTube::new();
    let res = client
        .query()
        .search("archlinux hyperland")
        .lang(Language::Spanish) // Optional: Specify language
        .location(Country::Spain) // Optional: Specify country
        .send::<VideoItem>()
        .await?;
    
    // Accessing search results
    println!("Estimated results: {:?}", res.estimated_results);
    
    for item in res.paginator.items() {
        println!("ID: {}", item.id);
        println!("Title: {}", item.title);
        println!("Channel: {}", item.channel.name);
        // ... and other fields
    }
    Ok(())
}
```

### Handling Pagination

Search results and other listings may be paginated. You can use the `paginator` to fetch more results.

```rust
let mut paginator = res.paginator;
while let Some(items) = paginator.next_page().await? {
    // Process the new items
    for item in items {
        println!("Next video title: {}", item.title);
    }
}
```

<div align="center">
    <img width="400" src="https://i.imgur.com/UGg0ePe.png" alt="img">
</div>
