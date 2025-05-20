SELECT
    vin
FROM
    `auto.echo_park`
WHERE
    invocationID = '1747479604'

EXCEPT DISTINCT

SELECT
    vin
FROM
    `auto.echo_park`
WHERE
    invocationID = '1747566004';





    WITH MissingVins AS (
    SELECT
        vin
    FROM
        `auto.echo_park`
    WHERE
        invocationID = '1747479604'

    EXCEPT DISTINCT

    SELECT
        vin
    FROM
        `auto.echo_park`
    WHERE
        invocationID = '1747566004'
)

WITH MissingVins AS (
    SELECT
        vin
    FROM
        `auto.echo_park`
    WHERE
        invocationID = '1747479604'

    EXCEPT DISTINCT

    SELECT
        vin
    FROM
        `auto.echo_park`
    WHERE
        invocationID = '1747566004'
)

SELECT
    t1.*
FROM
    `auto.echo_park` t1
JOIN
    MissingVins mv ON t1.vin = mv.vin
WHERE
    t1.invocationID = '1747479604';




    SELECT
    fixedFee__amount,
    invocationID
FROM
    `finance.revolut_fees`
WHERE
    invocationID = '1747656182'
    OR invocationID = '1747569786'
ORDER BY
    invocationID,
    fixedFee__amount;



    SELECT
    minimum_price,
    maximum_price,
    L3Name,
    L4Name,
    originalPrice,
    star1RatingPct,
    star2RatingPct,
    star3RatingPct,
    star4RatingPct,
    star5RatingPct,
    COUNT(*) as row_count
FROM
    `your_project_id.your_dataset_id.your_table_name` -- REPLACE with your actual full table path!
WHERE
    invocationID = '1747641604' -- The current failing invocation ID
GROUP BY
    minimum_price,
    maximum_price,
    L3Name,
    L4Name,
    originalPrice,
    star1RatingPct,
    star2RatingPct,
    star3RatingPct,
    star4RatingPct,
    star5RatingPct; -- Group by all affected columns