use regex::Regex;

#[derive(Default, Debug)]
pub struct Build {
    pub season: usize,
    pub patch: Option<f32>,
    pub netcl: usize,
}

pub fn get_build(useragent: &str) -> Option<Build> {
    let regex = Regex::new(r"[^\w=](\d{1}|\d{2}).(\d{2}|\d{1}).*-(\d{8}|\d{7})|-(\d{7})").ok()?;
    let captures = regex.captures(useragent)?;

    // sorry for the formatting cargo fmt is skunked
    let netcl: usize = match captures.get(3) {
        Some(netcl) => netcl,
        None => captures.get(4)?,
    }
    .as_str()
    .parse()
    .ok()?;

    let season = if netcl < 3807424 {
        1
    } else if netcl < 3901517 {
        2
    } else {
        captures.get(1)?.as_str().parse().ok()?
    };

    let patch = if let Some(patch) = captures.get(2) {
        Some(format!("{}.{}", season, patch.as_str()).parse().ok()?)
    } else {
        None
    };

    Some(Build {
        season,
        patch,
        netcl,
    })
}
