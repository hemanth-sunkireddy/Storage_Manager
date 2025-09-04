# Storage Manager

### Questions - 25/08

1. **Handling Pages:** When inserting a row into a table, the DBMS checks if the current page has enough free space; if not, it allocates a new page from the free list. But if the row is larger than a single page, how is it handled?

### Postgres

![Postgres Page](assets/25-08/PostGres_Page.png)
![Postgres Page Layout](assets/25-08/PostGres_PageLayout.png)
![Page Layout](assets/25-08/PageLayout.png)

### Page Header Layout

![Page Header Layout](assets/25-08/PageHeaderLayout.png)

### Page Header Data Example

![Demo](assets/25-08/Demo.png)

*When a table is created, no disk pages are allocated initially; pages are created only when rows are inserted.

### Each Tuple Size

- _First Insert:_ 8192 → 8152 = 40 bytes
- _Second Insert:_ 8152 → 8120 = 32 bytes
- _Third Insert:_ 8120 → 8080 = 40 bytes
  ![Tuple Sizes](assets/25-08/TableRowsTupleSizes.png)
  Q. Int is 4 bytes and name is (1 length header + actual data bytes) ~8 bytes. Then what is remaining 23 bytes?
  ![Tuple Header Description](assets/25-08/TupleHeaderDescription.png)
  ![Tuple Header](assets/25-08/TupleHeader.png)

### Inserted Rows tuple details

![Tuple Details](assets/25-08/RowsTupleDetail.png)
* `lp_len` is actual data size in a tuple (tupleHeader + tupleData).
- Actual tuple data stored from `t_hoff` and size of data is `lp_len`. 
*Ex:* for first row:
  - `lp_len` - `t_hoff` = 34 - 24 = **10 bytes** ( 4 bytes for int + 6 bytes for TEXT - ("Alice" - 5 bytes + 1 length header) )

**Q:**  _First Tuple:_ 8192 → 8152 = 40 bytes, took 40 bytes. What are extra 6 bytes?
* Postgres adds **Padding** to each tuple.
PostgreSQL stores tuples aligned to **MAXALIGN** (usually 8 bytes), so the actual space allocated on the page = next multiple of MAXALIGN.

**Q.** Why 1 byte length header for each value of text (Variable length datatype)?
## our question: row is larger than a single page, how is it handled?
![Toast](assets/25-08/Toast.png)

### Q. When does TOAST occur?
![Toast Call](assets/25-08/ToastCall.png)
### Q. Do every table have a Toast Table associated to it?
No
![Toast Condition](assets/25-08/ToastCondition.png)
### Storing Toasted Data
![Toast Data Store](assets/25-08/ToastData.png)

### Toasted Data Example
![Toast Data Size](assets/25-08/ToastDataSize.png)
**Q:** Even after adding ~10KB (>threshold), why data is not toasted?
**Compression**
![Compression](assets/25-08/Compression.png)

### Adding Large Tuple with Random values
![Add Large Tuple](assets/25-08/LargeTupleAddition.png)
* Size of tuple: 32 * 500 = 16000 bytes
![fifth Row Addition](assets/25-08/fifthRowOutput.png)
![Logical Size of Fifth Row](assets/25-08/LogicalSizeOfFifthRow.png)
![Compression 2](assets/25-08/Compression2.png)

### Adding More than 2KB
![Add more than 2KB](assets/25-08/SixthRowAddition.png)
* Size of tuple: 32 * 100000 = 3.2 MB
* Physical Size: 36 KB
![Physical size of sixth Row](assets/25-08/PhysicalSizeOfsixthRow.png)
**Q.** How 36KB stored?
![Final Data Storage](assets/25-08/FinalDataStorage.png)
**Q.** What do that 46 bytes contain for last row? How original row maps to TOAST table?
  - Tuple Header: 23 bytes
  - int: 4 bytes
  - name length header: 1 byte
  - name datum: 18 bytes
![Datum](assets/25-08/PointerDatum.png)
So, in the main table page, only stores pointer datum to the toast table.
**Q.** How the data stored in Toasted Table?
Details [here](#storing-toasted-data)



### Observations
* When a table is created, no disk pages are allocated initially; pages are created only when rows are inserted.
* For variable length data structure, `1 byte` length header is storing along with data in the tuple.
* PostgreSQL stores small values inline; large variable-length values may be TOASTed.
* TOAST tables are created only for tables with TOAST-able (variable-length) columns.
* Postgres Compress data only when data is greater than threshold.
* TOAST triggers when a value cannot fit inline after compression; highly compressible data may stay inline.
* TOAST tables are created only for tables with TOAST-able (variable-length) columns.

### Conclusion
* When building our storage manager, few points to consider
    - **Variable-Length Data:** Handle using a mechanism similar to TOAST.
    - **Compression:** Compression algorithm
_____

2. How much does modern database systems(opensource) -mysql or postgres depend on File Systems of OS?
3. What should be the requirements from a file manager towards supporting a dbms?
4. How many of this requirements are supported by modern database systems?
5. Understand from physical level to file system - what are the inherent constraints current solutions have?
6. If we create file system, What are the disadvantages of our creation. Why is this best solution than existing?
7. How can we build a **simple file manager** that maintains lookup tables and counters, and can create a file with n pages (or a fixed size like 10 MB), allowing new pages to be added?

---

### Previous Questions - 20/08

1. When we run create database, what happens? Some metadata is stored; beyond that is some space allotted to it.

<table>
  <tr>
    <td width="50%">
      <h3>Newly Installed SQL</h3>
      <img src="assets/NewSQL.png" width="100%"/>
      <ul>
        <li><b>Buffer Pool:</b> total number of pages that InnoDB can hold.</li>
        <li>Each page = 16 KB.</li>
        <li>Buffer pool size = 8191 * 16 KB ≈ 128 MB.</li>
        <li><b>Old Database Pages:</b> number of old pages in LRU.</li>
        <li>InnoDB splits into Young list & Old list.</li>
        <li><b>LRU Length:</b>1059 = Database pages len.</li>
      </ul>
    </td>
    <td width="50%">
      <h3>After Creating Database</h3>
      <img src="assets/AfterDBCreation.png" width="100%"/>
      <ul>
        <li><b>Free buffers:</b> 7132 → 7063 (69 pages allocated).</li>
        <li><b>Background pre-allocation:</b> InnoDB reserves pages for extents.</li>
        <li><b>Extent:</b> 64 pages (1 MB).</li>
        <li>5 pages used for metadata + indexes.</li>
        <li><b>Database Pages: </b> 1059 -> 1128 = 69 pages.</li>
        <li><b>LRU Length:</b>1128 pages</li>
      </ul>
    </td>
  </tr>
</table>

---

2. When we create a table, what happens, some metadata is stored, and beyond that is some space allotted to it.

<table>
  <tr>
    <td width="50%">
      <h3>Before Creating Table</h3>
      <img src="assets/BeforeTableCreation.png" width="100%"/>
      <ul>
        <li>Modified Db pages 0 - Dirty pages</li>
      </ul>
    </td>
    <td width="50%">
      <h3>After Creating Table</h3>
      <img src="assets/AfterTableCreation.png" width="100%"/>
      <ul>
        <li>Modified DB pages: 1</li>
        <li>After Flushing to Disk, modified db pages goes to 0.</li>
        <li>Around 6 to 10 free pages used when table created.</li>
      </ul>
    </td>
  </tr>
</table>

---

3. When Inserting in a table

<table>
  <tr>
    <td width="25%">
      <h3>Before Insertion</h3>
      <img src="assets/AfterTableCreation.png" width="100%"/>
    </td>
    <td width="25%">
      <h3>After 1 row INSERTION</h3>
      <img src="assets/OneRowInsertion.png" width="100%"/>
    </td>
    <td width="25%">
      <h3>2nd row insertion</h3>
      <img src="assets/TwoRowInsertion.png" width="100%"/>
    </td>
    <td width="25%">
      <h3>3rd row INSERTION</h3>
      <img src="assets/ThirdRowInsertion.png" width="100%"/>
    </td>
  </tr>
</table>

---

4. How are these pages managed, and how does one access the page?

<table>
  <tr>
    <td width="25%">
      <h3>Before Insertion</h3>
      <img src="assets/BeforeReadWrite.png" width="100%"/>
    </td>
    <td width="25%">
      <h3>After 1 row INSERTION</h3>
      <img src="assets/OneWrite.png" width="100%"/>
    </td>
    <td width="25%">
      <h3>2nd row insertion</h3>
      <img src="assets/TwoWrite.png" width="100%"/>
    </td>
    <td width="25%">
      <h3>3rd row INSERTION</h3>
      <img src="assets/ThreeWrite.png" width="100%"/>
    </td>
  </tr>
</table>

- At every time 6 to 10 pages are writing to the disk - Redo/undo logs + actual page + indexing pages.
- No reads from disk when already exists in cache - read from cache, update in cache, then update to disk.

---

5. How is data inserted into the page? There is an application data structure, and a disk block from disk. Are changes made in the application data structure and copied to the disk block in the main memory buffer, or directly the main memory buffer copy of the disk block modified directly?

- Each process uses a thread that points to the buffer pool page instead of making a copy.
- Updates are applied directly to the cached page, with locks ensuring other threads cannot access it simultaneously.

![Locks](assets/Locks.png)

---

# Tasks - 18/08

A - whether OS should manage database space or DBMS must manage its chunk of space.  
![Database and OS](assets/Database_OS.png)  
![Disadvantages of Block Level](assets/Disadvantages_BlockLevel.png)  
![OS File System](assets/FileSystem_OS.png)

B - kernel implementation of page/disk block-oriented file access - page by page.

- If DBMS manages its chunk of pages then -
  - Data structures are created to store different page types in our **Storage Engine**.
  - A buffer pool is built to manage these pages.
  - The server requests data from disk through OS file system operations.
  - We create files and directories in the disk - We call OS to get the data from this files using file system.
