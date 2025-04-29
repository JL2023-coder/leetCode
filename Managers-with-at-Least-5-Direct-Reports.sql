# Write your MySQL query statement below
SELECT
    t2.name
FROM
    Employee AS t1
JOIN Employee AS t2 ON
    t1.managerId = t2.id
GROUP BY 
    t1.managerId
HAVING
    COUNT(*) >= 5
