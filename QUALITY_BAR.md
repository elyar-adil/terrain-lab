# Satellite Realism Quality Bar

## North-star goal

Given a generated image and a real satellite crop of an uninhabited natural area at the same ground sampling distance, geographic scale, season and biome, a human observer should not be able to reliably determine which image is generated.

The generator must be able to produce terrain comparable to randomly selected real-world examples, rather than succeeding only on hand-picked mountain scenes.

## Required coverage

- Mountain ranges, hills, plains, plateaus, coasts and archipelagos
- Arid, temperate and glacial climates
- Rivers, tributaries, lakes, wetlands, floodplains, deltas and coastlines
- Forest, grassland, scrub, bare soil, exposed rock, sediment, snow and shallow water
- Multiple spatial scales, from regional structure to sensor-scale surface variation

## Evaluation protocol

- Match image size, ground sampling distance, sun direction, season and biome before comparison.
- Use blind A/B tests with randomized real and generated crops; the long-term target is classification accuracy statistically indistinguishable from chance.
- Include randomly sampled test regions. Do not evaluate only curated references or successful seeds.
- Measure drainage density, elevation spectrum, land-cover patch sizes, coastline complexity, color distribution, spatial frequency and repeated-pattern artifacts.
- Reject results containing grid-aligned rivers, periodic noise, uniform biome coverage, artificial borders, impossible drainage or repeated texture motifs.

## Rendering requirements

- Satellite realism takes priority over dramatic terrain presentation.
- 3D preview uses true 1:1 vertical scale by default.
- Final orthographic output models ground sampling distance, atmospheric scattering, sensor point-spread, local contrast, sharpening and compression consistently.
- Procedural terrain, hydrology and land cover must be semantically valid before sensor effects are applied.

## Data policy

Google Maps may be used as a manually inspected visual benchmark, subject to its terms. Do not scrape, redistribute or train on Google imagery without permission.

Automated reference datasets should use legally reusable sources such as Copernicus Sentinel, Landsat, USGS/NAIP and other clearly licensed remote-sensing products.

Generated files should retain synthetic provenance metadata even when their pixels are visually realistic.
