/*
--- Day 1: Report Repair ---

After saving Christmas five years in a row, you've decided to take a vacation at a nice resort on a tropical island. Surely, Christmas will go on without you.

The tropical island has its own currency and is entirely cash-only. The gold coins used there have a little picture of a starfish; the locals just call them stars. None of the currency exchanges seem to have heard of them, but somehow, you'll need to find fifty of these coins by the time you arrive so you can pay the deposit on your room.

To save your vacation, you need to get all fifty stars by December 25th.

Collect stars by solving puzzles. Two puzzles will be made available on each day in the Advent calendar; the second puzzle is unlocked when you complete the first. Each puzzle grants one star. Good luck!

Before you leave, the Elves in accounting just need you to fix your expense report (your puzzle input); apparently, something isn't quite adding up.

Specifically, they need you to find the two entries that sum to 2020 and then multiply those two numbers together.

For example, suppose your expense report contained the following:

1721
979
366
299
675
1456

In this list, the two entries that sum to 2020 are 1721 and 299. Multiplying them together produces 1721 * 299 = 514579, so the correct answer is 514579.

Of course, your expense report is much larger. Find the two entries that sum to 2020; what do you get if you multiply them together?

*/
static EXPENESES: [u16; 200] = [
    1782, 1344, 1974, 1874, 1800, 1973, 1416, 1952, 1982, 1506, 1642, 1514, 1978, 1895, 1747, 1564,
    1398, 1683, 1886, 1492, 1629, 1433, 295, 1793, 1740, 1852, 1697, 1471, 1361, 1751, 1426, 2004,
    1763, 1663, 1742, 1666, 1733, 1880, 1600, 1723, 1478, 1912, 1820, 1615, 1875, 1547, 1554, 752,
    1905, 1368, 954, 1425, 1391, 691, 1835, 744, 1850, 1713, 1995, 1926, 1817, 1774, 1986, 2010,
    1427, 1609, 1927, 1362, 1420, 1722, 1590, 1925, 1617, 1434, 1826, 1636, 1687, 1946, 704, 1797,
    1517, 1801, 1865, 1963, 1828, 1829, 1955, 1832, 1987, 1585, 1646, 1575, 1351, 1345, 1729, 1933,
    1918, 1902, 1490, 1627, 1370, 1650, 1340, 1539, 1588, 1715, 1573, 1384, 1403, 1673, 1750, 1578,
    1831, 1849, 1719, 1359, 2008, 1837, 1958, 480, 1388, 1770, 1999, 1066, 1730, 1541, 1802, 1962,
    1891, 1816, 1505, 1665, 1551, 1954, 1378, 1998, 1612, 1544, 1953, 1502, 1888, 1655, 1614, 1903,
    1675, 1498, 1653, 1769, 1863, 1607, 1945, 1651, 1558, 1777, 1460, 1711, 1677, 1988, 1441, 1821,
    1867, 1656, 1731, 1885, 1482, 1439, 1990, 1809, 1794, 1951, 1858, 1969, 509, 1486, 1971, 1557,
    1896, 1884, 1834, 1814, 1216, 1997, 1966, 1808, 1754, 1804, 1684, 2001, 1699, 1781, 1429, 1322,
    1603, 1596, 1823, 1700, 1552, 1352, 1621, 1669,
];

fn part1() {
    let mut sorted = EXPENESES.clone();
    sorted.sort();

    let mut i = 0;
    let mut j = sorted.len() - 1;

    while i < j {
        let sum = sorted[i] + sorted[j];
        if sum == 2020 {
            break;
        } else if sum > 2020 {
            j -= 1;
        } else {
            // sum < 2020
            i += 1;
        }
    }

    let prod = sorted[i] as u32 * sorted[j] as u32;
    println!("{}*{}={}", sorted[i], sorted[j], prod);
}

fn main() {
    part1();
}
