use std::sync::LazyLock;

use anyhow::{bail, Context, Result};
use csv::StringRecord;
use include_dir::{include_dir, Dir};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Post {
  pub title: String,
  pub author: String,
  pub url: String,
}

pub static POSTS: LazyLock<Vec<Post>> = LazyLock::new(|| {
  serde_json::from_str(include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/posts.json"
  )))
  .unwrap()
});

#[derive(Debug, Clone, PartialEq)]
pub struct Score {
  pub owner: String,
  pub post_id: usize,
  pub literary: f64,
  pub thinking: f64,
  pub mozheng: f64,
}

impl Score {
  fn parse_csv(owner: String, record: &StringRecord) -> Result<Self> {
    let title = record[0].trim();
    let post = POSTS
      .iter()
      .enumerate()
      .filter(|(_, p)| p.title == title)
      .next()
      .with_context(|| format!("title `{}` not found", title))?;

    let post_id = post.0;
    let literary = record[1].to_string().parse()?;
    let thinking = record[2].to_string().parse()?;
    let mozheng = record[3].to_string().parse()?;

    Ok(Score {
      owner,
      post_id,
      literary,
      thinking,
      mozheng,
    })
  }

  fn scale_dimension(mut scores: Vec<f64>) -> Result<Vec<f64>> {
    const EPS: f64 = 1e-6;

    // Scale to [-1, 1]
    let min = *scores
      .iter()
      .min_by(|a, b| a.partial_cmp(b).unwrap())
      .unwrap();
    let max = *scores
      .iter()
      .max_by(|a, b| a.partial_cmp(b).unwrap())
      .unwrap();

    if min == max {
      return Ok(scores.into_iter().map(|_| 0.).collect());
    }

    scores = scores
      .into_iter()
      .map(|f| ((f - min) / (max - min) * 2. - 1.) * (2. / 3.))
      .collect();

    // Ternary search
    let mut l = 0.;
    let mut r = 1.;
    let mut res = 0.5;
    while r - l > EPS {
      let one_third = (r - l) / 3.;
      let mid_l = l + one_third;
      let mid_r = mid_l + one_third;

      let res_l = check(scores.clone(), mid_l);
      let res_r = check(scores.clone(), mid_r);

      if res_l < res_r {
        res = mid_l;
        r = mid_r;
      } else {
        res = mid_r;
        l = mid_l;
      }
    }

    const MAX_ERROR: f64 = 1e-3;
    let error = check(scores.clone(), res);

    if error > MAX_ERROR {
      bail!("Scaling failed: error {} is too large", error);
    }

    scores = scores.into_iter().map(|x| transform(x, res)).collect();
    // console_dbg!(scores);
    return Ok(scores);

    fn transform(x: f64, t: f64) -> f64 {
      let pow = -f64::ln(1. - t);
      x.signum() * x.abs().powf(pow)
    }

    fn pow_avg(scores: &[f64]) -> f64 {
      scores.iter().map(|s| s.powi(2)).sum::<f64>() / scores.len() as f64
    }

    fn check(scores: Vec<f64>, t: f64) -> f64 {
      const TARGET_VARIANCE: f64 = 1. / 3.;
      let transformed: Vec<_> = scores.into_iter().map(|x| transform(x, t)).collect();

      (pow_avg(&transformed) - TARGET_VARIANCE).abs()
    }
  }

  fn scale(scores: &mut [Score]) -> Result<()> {
    let mut s_literary: Vec<_> = scores.iter().map(|s| s.literary).collect();
    let mut s_thinking: Vec<_> = scores.iter().map(|s| s.thinking).collect();
    let mut s_mozheng: Vec<_> = scores.iter().map(|s| s.mozheng).collect();

    s_literary = Self::scale_dimension(s_literary)?;
    s_thinking = Self::scale_dimension(s_thinking)?;
    s_mozheng = Self::scale_dimension(s_mozheng)?;

    for i in 0..scores.len() {
      scores[i].literary = s_literary[i];
      scores[i].thinking = s_thinking[i];
      scores[i].mozheng = s_mozheng[i];
    }

    Ok(())
  }

  pub fn sum(&self) -> f64 {
    self.literary + self.thinking + 1.5 * self.mozheng
  }
}

pub static SCORES: LazyLock<Vec<Score>> = LazyLock::new(|| {
  static DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/scores");

  DIR
    .files()
    .filter(|f| {
      f.path()
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .ends_with(".csv")
    })
    .map(|f| {
      let owner = f.path().file_name().unwrap().to_str().unwrap();
      let owner = owner[..owner.len() - 4].to_string();

      let mut reader = csv::Reader::from_reader(f.contents());
      let mut scores = Vec::new();

      for res in reader.records() {
        let record = res.unwrap();
        scores.push(Score::parse_csv(owner.clone(), &record).unwrap());
      }

      Score::scale(&mut scores).unwrap();
      scores.into_iter()
    })
    .flatten()
    .collect()
});

#[derive(Debug, Clone, PartialEq)]
pub struct PostWithScores {
  pub id: usize,
  pub post: Post,
  pub scores: Vec<Score>,
}

impl PostWithScores {
  pub fn literary_avg(&self) -> f64 {
    let sum: f64 = self.scores.iter().map(|s| s.literary).sum();
    let n = self.scores.len();
    sum / (n as f64)
  }

  pub fn thinking_avg(&self) -> f64 {
    let sum: f64 = self.scores.iter().map(|s| s.thinking).sum();
    let n = self.scores.len();
    sum / (n as f64)
  }

  pub fn mozheng_avg(&self) -> f64 {
    let sum: f64 = self.scores.iter().map(|s| s.mozheng).sum();
    let n = self.scores.len();
    sum / (n as f64)
  }

  pub fn sum_avg(&self) -> f64 {
    let sum: f64 = self.scores.iter().map(|s| s.sum()).sum();
    let n = self.scores.len();
    sum / (n as f64)
  }

  pub fn literary_var(&self) -> f64 {
    let avg = self.literary_avg();
    let sum: f64 = self.scores.iter().map(|s| (s.literary - avg).powi(2)).sum();
    let n = self.scores.len();
    sum / (n as f64)
  }

  pub fn thinking_var(&self) -> f64 {
    let avg = self.thinking_avg();
    let sum: f64 = self.scores.iter().map(|s| (s.thinking - avg).powi(2)).sum();
    let n = self.scores.len();
    sum / (n as f64)
  }

  pub fn mozheng_var(&self) -> f64 {
    let avg = self.mozheng_avg();
    let sum: f64 = self.scores.iter().map(|s| (s.mozheng - avg).powi(2)).sum();
    let n = self.scores.len();
    sum / (n as f64)
  }

  pub fn sum_var(&self) -> f64 {
    let avg = self.sum_avg();
    let sum: f64 = self.scores.iter().map(|s| (s.sum() - avg).powi(2)).sum();
    let n = self.scores.len();
    sum / (n as f64)
  }
}

pub static POSTS_WITH_SCORES: LazyLock<Vec<PostWithScores>> = LazyLock::new(|| {
  let mut res: Vec<_> = POSTS
    .iter()
    .cloned()
    .enumerate()
    .map(|(id, post)| PostWithScores {
      id,
      post,
      scores: Vec::new(),
    })
    .collect();

  for score in SCORES.iter() {
    res[score.post_id].scores.push(score.clone());
  }

  res
});
