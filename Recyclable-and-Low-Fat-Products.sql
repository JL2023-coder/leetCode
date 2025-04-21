# Write your MySQL query statement below
SELECT
    p.product_id
FROM
    products AS p
WHERE 
    low_fats = "Y" AND 
    recyclable = "Y"
