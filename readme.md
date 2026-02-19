# Інструкція до виконання практичних робіт №3–10

Дисципліна: **Системне програмування / Мови системного програмування (Rust)**

---

## 1. Загальні вимоги

Практичні роботи виконуються індивідуально.
Кожне завдання повинно бути реалізоване у вигляді окремого Rust-модуля з тестами.
Результат роботи повинен бути підтверджений локальним проходженням тестів та проходженням тестів на платформі **HackerRank**.
Код повинен бути чистим, без попереджень компілятора (warnings), без коментарів ChatGPT.

---

## 2. Структура проєкту

1. Кожна задача повинна бути розміщена у власному файлі.
2. Заборонено розміщувати декілька задач в одному файлі.
3. [Орієнтир](https://github.com/djnzx/rust-practice2/tree/master/src/hackerrank) по структурі директорій

Рекомендована структура:

```
src/
  hackerrank/
    task3.rs
    task4.rs
    task5.rs
    ...
```

---

## 3. Алгоритм виконання кожної задачі

### Крок 1. Реалізація задачі

1. Перейти за відповідним посиланням на задачу.
2. Зрозуміти вимоги до задачі.
3. Реалізувати розв’язок задачі відповідно до вимог умови.
4. Оформити відповідно [прикладу](https://github.com/djnzx/rust-practice2/blob/master/src/hackerrank/task0.rs#L2).

---

### Крок 2. Додавання тестів

1. До кожної задачі обов’язково додати unit-тест.
2. Тест повинен перевіряти коректність реалізації.
3. [Приклад](https://github.com/djnzx/rust-practice2/blob/master/src/hackerrank/task0.rs#L11) оформлення тесту:

---

### Крок 3. Перевірка локально

1. Запустити тести локально:

   ```
   cargo test
   ```

2. Зробити скріншот з локального компʼютера, де видно:
    * що всі тести пройшли.
    * відсутні warnings - Роботи з попередженнями компілятора (warning) не зараховуються.
    * [приклад](https://github.com/djnzx/rust-practice2/blob/master/src/hackerrank/results/0/rust2-example0-console-no-warning.png) коректного результату. 

3. Якщо є warnings — робота не зараховується.
   * [Приклад](https://github.com/djnzx/rust-practice2/blob/master/src/hackerrank/results/0/rust2-example0-console-warning.png) некоректного результату.

---

### Крок 4. Перевірка на платформі HackerRank

1. Завантажити рішення на платформу **HackerRank**.
2. Переконатися, що всі тести на сайті пройдені.
3. Зробити скріншот з результатом проходження. [Приклад](https://github.com/djnzx/rust-practice2/blob/master/src/hackerrank/results/0/rust2-example0-site-result.png).

---

### Крок 5. Завантаження у репозиторій

1. Додати змінені та нові файли у відповідні директорії.

2. Виконати:

    ```
    git add .
    git commit -m "..."
    git push
    ```

3. Завантажити зміни до репозиторію на **GitHub**.

4. Коментар до коміту повинен:

    * відображати зміст змін
    * містити номер практичної
    * мати сенс

[Приклад](https://github.com/djnzx/rust-practice2/commits/master/) історії комітів:

---

## 6. Критерії зарахування роботи

Робота зараховується лише якщо:

* задача реалізована коректно
* додані unit-тести
* всі локальні тести проходять
* відсутні warnings
* є скріншот локального результату
* є скріншот результату з HackerRank
* файли правильно розміщені в структурі
* коміт оформлений коректно

---

## 7. Типові помилки

* завантаження файлів через drag & drop, upload
* невідповідність структури директорій
* коміт з повідомленням типу: `update` або `fix`
* розміщення кількох задач в одному файлі
* відсутність тестів
* наявність warning
* відсутність скріншотів

## Список задач для вирішення

* Практична 3: https://www.hackerrank.com/challenges/staircase
* Практична 4: https://www.hackerrank.com/challenges/grading
* Практична 5: https://www.hackerrank.com/challenges/apple-and-orange
* Практична 6: https://www.hackerrank.com/challenges/kangaroo
* Практична 7: https://www.hackerrank.com/challenges/between-two-sets
* Практична 8: https://www.hackerrank.com/challenges/breaking-best-and-worst-records
* Практична 9: https://www.hackerrank.com/challenges/migratory-birds
* Практична 10: https://www.hackerrank.com/challenges/sock-merchant
* Практична 11: https://www.hackerrank.com/challenges/diagonal-difference
* Практична 12: https://www.hackerrank.com/challenges/birthday-cake-candles
* Практична 13: https://www.hackerrank.com/challenges/divisible-sum-pairs
* Практична 14: https://www.hackerrank.com/challenges/bon-appetit
* Практична 15: https://www.hackerrank.com/challenges/drawing-book
