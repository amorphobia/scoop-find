use phf::phf_map;

pub static BUCKETS: phf::Map<&'static str, &'static str> = phf_map! {
    "main" => "https://api.github.com/repos/ScoopInstaller/Main/git/trees/HEAD?recursive=1",
    "extras" => "https://api.github.com/repos/ScoopInstaller/Extras/git/trees/HEAD?recursive=1",
    "versions" => "https://api.github.com/repos/ScoopInstaller/Versions/git/trees/HEAD?recursive=1",
    "nirsoft" => "https://api.github.com/repos/kodybrown/scoop-nirsoft/git/trees/HEAD?recursive=1",
    "php" => "https://api.github.com/repos/ScoopInstaller/PHP/git/trees/HEAD?recursive=1",
    "nerd-fonts" => "https://api.github.com/repos/matthewjberger/scoop-nerd-fonts/git/trees/HEAD?recursive=1",
    "nonportable" => "https://api.github.com/repos/TheRandomLabs/scoop-nonportable/git/trees/HEAD?recursive=1",
    "java" => "https://api.github.com/repos/ScoopInstaller/Java/git/trees/HEAD?recursive=1",
    "games" => "https://api.github.com/repos/Calinou/scoop-games/git/trees/HEAD?recursive=1",
};
