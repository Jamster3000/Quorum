This holds all schemas required for this project.

`initial.surql` Is the main surrealDB to initiate and create the full database table.

> Note: To keep the settings take up the least amount of storage space possible, in the application layer it will be presented as a long integer.
> This could be presented something like `010834` Where every 2 numbers will represent a settings value. Each number will be in the same order that settings are ordered in the application layer. 
> For example, 01 might represent the first setting, 08 might be the second setting, etc. For an integer setting, this works directly, any number from 00 - 99. For a true/false setting `00` and `01`.
> There is consideration for storing text in each settings table and linking that with the integer settings number but this is still only a consideration until open string settings are directly needed.