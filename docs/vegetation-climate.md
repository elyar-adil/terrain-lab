# Vegetation follows the terrain

A town's trees, understory and lawn are not a fixed list. They are a function of
the climate and landform of the place the terrain simulation put the town.

## The data path

```
terrain-core  TerrainData { temperature, moisture, height, snow, ... }
      |  infrastructure::climate::site_climate   (mean over a 1.5 km window at the town centre;
      |                                           nearest-sea search over 80 km)
      v
urban::SiteClimate  { temperature_c, moisture, elevation_m, snow, coast_km }
      |  stored on ModernCity.climate (serde default = warm-temperate inland lowland)
      v
city-scene::scene::build_scene
      |-- trees::zones::environment_for(climate)  -> Zone + keep + size + dry
      |      trees::plant::plant_in_climate        (species allocation per role, thinning, height)
      `-- bake::standard_set_in(.., climate)        (lawn texture: straw/soil <-> deep green)
```

Plans made without terrain (`dump_scene` with no `--from`, tests) carry the
default `SiteClimate`, which falls in the `warm-temperate` zone whose allocation
is the planting the ChineseModern style always had.

`dump_scene --climate temp_c,moisture,elevation_m,coast_km[,snow]` plants a
plan as if it stood in that climate, for inspection.

## Which terrain variable drives which choice

| terrain field | `SiteClimate` | drives |
|---|---|---|
| `temperature` (deg C, `21 - 10*lat - 0.0062*h`) | `temperature_c` | zone (cold northeast / temperate north / warm temperate / subtropical) |
| `moisture` (0..1) | `moisture` | zone (arid vs humid); tree survival `keep`; tree height `size`; lawn lushness; understory and tuft count; straw tint of tufts and shrubs |
| `height` (m) | `elevation_m` | plateau/alpine zone (ramps in 1900-2700 m); arid zone is capped below it |
| `snow` (0..1) | `snow` | height cut (shorter trees where snow persists) |
| `height < 58 m` is sea (terrain-core boundary water) | `coast_km` | coastal zones when the sea is within about 18-40 km |

A zone is a region of (temperature, moisture, elevation, coast distance) space
described by trapezoids; the best-scoring zone (membership product times a
specificity priority) plants the town. See `crates/city-scene/src/trees/zones.rs`:
the tables are data and there are no species conditionals.

## Zone table

Species keys in parentheses. The allocation per role (street, motorway, median,
park, compound, waterfront, farm, orchard, island) is in `zones.rs`.

| zone | stand-in cities (mean annual deg C) | main species |
|---|---|---|
| cold-northeast 东北寒温带 | Harbin 4, Changchun 6 | 榆 (yu-shu), 杨 (yang-shu), 柳 (liu-shu), 白桦 (bai-hua), 樟子松 (song-shu), 云杉 (sha-shu) |
| temperate-north 华北暖温带 | Beijing 13, Taiyuan 10, Shenyang 8 | 国槐 (huai-shu), 杨, 白蜡 (bai-la), 栾树 (luan-shu), 银杏 (yin-xing), 油松 (song-shu), 玉兰 (yu-lan), 柳 |
| warm-temperate 黄淮江淮过渡带 | Nanjing 16, Zhengzhou 15, Xi'an 14 | 国槐, 悬铃木 (wu-tong), 香樟 (xiang-zhang), 银杏, 白蜡, 杨 |
| humid-subtropical 亚热带湿润区 | Chengdu 16, Wuhan 17, Fuzhou 20 | 香樟, 榕树 (rong-shu), 悬铃木, 栾树, 合欢 (he-huan), 桂花 (gui-hua), 玉兰 |
| coast-warm 南方滨海 | Xiamen 21, Guangzhou 22 | 榕树, 香樟, 桂花, 合欢, 紫薇 (zi-wei), 黑松 (song-shu) |
| coast-cool 北方滨海 | Qingdao 13, Dalian 11 | 国槐, 悬铃木, 雪松 (xue-song), 黑松 (song-shu), 杨, 银杏 |
| arid-northwest 西北干旱区 | Urumqi 7, Lanzhou 10, Yinchuan 9 | 新疆杨/杨 (yang-shu), 榆, 胡杨 (hu-yang), 沙枣 (sha-zao) |
| plateau 高原与高山 | Xining 6, Lhasa 8 | 杨 (藏川杨), 青海云杉 (sha-shu), 祁连圆柏 (long-bai), 柳, 沙枣 |

Per-zone `keep` and `size` (before the moisture swing): plateau 0.55 / 0.62, arid
0.60 / 0.72, cold-northeast 0.85 / 0.88, temperate-north 0.92 / 0.96, warm and
subtropical 1.0 / 1.0-1.04, coastal 0.9-0.95 / 0.86-0.88. Street, median and
island trees thin by `sqrt(keep)` (they are the watered strips), the waterfront by
`(keep+1)/2`, parcels by `keep`.

## References used

Text only; no image was copied or committed. The species-to-region mapping is
from the standard vegetation regionalisation and published urban tree lists, not
from a measurement:

* 《中国植被》 (Wu Zhengyi, 1980) vegetation regions and 《中国树木志》 for crown
  form, leaf and bark of each species.
* Koppen-Geiger map of China (Peel et al. 2007) for the temperature/wetness bands.
* Street-tree practice by city: Beijing (国槐 is the city tree, with 白蜡, 栾树, 银杏,
  油松), Nanjing (more than 80 000 悬铃木 on its streets, planted along Zhongshan
  Avenue from 1928; Forests 14(9) 2023), Harbin (榆, 杨, 云杉), Qingdao (雪松, 黑松,
  悬铃木), Urumqi (新疆杨, 榆), Xining (青海云杉, 祁连圆柏), Lhasa (藏川杨),
  Guangzhou and Xiamen (榕树, 香樟).
* CJJ 75 (tree-pit spacing on streets, already in the planting code).

## What is not modelled (and what stands in)

* Palms (棕榈, 大王椰子, 蒲葵, 椰子), casuarina (木麻黄), bamboo (竹), litchi and
  mango, tamarisk (柽柳), mangrove and reeds on tidal flats, 沙棘 and 梭梭 shrubs, cushion plants
  and alpine meadow have no canopy form or prototype yet. The warm coast and
  subtropical zones use 榕树 and 香樟; alpine and arid zones use the lawn texture
  and thinning to look dry and bare. Wind-bent crowns are not modelled.
* There is no season in the renderer: leaf cards are baked in summer colour.
  Species carry an autumn colour in the table but nothing selects it, so
  "winter-leafless versus evergreen by climate and season" cannot be shown
  until a season setting exists.
* Elevation-dependent treeline is only approximated by the plateau zone's `size`
  and `keep`; there is no continuous treeline.
