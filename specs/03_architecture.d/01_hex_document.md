
### Архитектура In-Memory модели (`HexDocument`)

Для реализации разреженной памяти (sparse memory) с эффективным доступом лучше всего подходит подход **"Список чанков" (Chunked Buffer)** или **Page-based storage**.

#### 1. Структуры данных (Rust Reference)

Мы будем использовать `BTreeMap` для хранения кусков данных, так как он позволяет быстро находить нужный кусок по адресу (поиск ближайшего ключа, меньшего или равного искомому).

```rust
use std::collections::BTreeMap;

// Один непрерывный кусок данных
struct MemoryChunk {
    start_address: u64,
    data: Vec<u8>,
}

// Метаданные для конкретного адреса (комментарии, типы)
struct CellMetadata {
    comment: Option<String>,
    type_hint: Option<String>, // например "u32", "utf8"
    struct_field: Option<String>, // имя поля, если это часть структуры
}

// Основной объект документа
pub struct HexDocument {
    // Данные: Адрес начала чанка -> Байты чанка
    // BTreeMap позволяет эффективно искать чанк, содержащий произвольный адрес
    chunks: BTreeMap<u64, Vec<u8>>,
    
    // Метаданные: Адрес -> Данные (разряженная карта)
    metadata: BTreeMap<u64, CellMetadata>,
    
    // Текущие настройки отображения для этого документа
    options: HexOptions, 
    
    // Кэш общей длины или границ (min_addr, max_addr)
    bounds: (u64, u64),
}
```

#### 2. API доступа (Trait-based)

Чтобы работать с данными как с `Vec`, но умнее, реализуем типажи для чтения/записи примитивов.

```rust
impl HexDocument {
    // Базовые операции
    pub fn get_byte(&self, addr: u64) -> Option<u8> {
        // Логика:
        // 1. Найти чанк, у которого start_address <= addr
        // 2. Проверить, попадает ли addr в диапазон [start, start + len)
        // 3. Вернуть байт или None (если дырка)
    }

    pub fn set_byte(&mut self, addr: u64, value: u8) {
        // Логика:
        // 1. Если попадаем в существующий чанк -> обновляем.
        // 2. Если попадаем в "дырку" рядом с чанком -> расширяем чанк.
        // 3. Если далеко от всего -> создаем новый чанк.
        // 4. (Опционально) Мержим соседние чанки, если они соприкоснулись.
    }
    
    // Работа с типами (использует options.endian)
    pub fn read_u32(&self, addr: u64) -> Option<u32> { ... }
    pub fn write_u32(&mut self, addr: u64, value: u32) { ... }
    
    // Вставка/Удаление (сдвиг адресов) - сложная операция для редактора
    pub fn insert_bytes(&mut self, addr: u64, data: &[u8]) { ... }
    pub fn delete_range(&mut self, start: u64, len: u64) { ... }
}
```

#### 3. Интеграция с потоковым парсером

Важный момент: парсер должен уметь работать в двух режимах.
1. **Streaming Mode:** Читает токены и сразу пишет в Output (для CLI).
2. **DOM Mode:** Читает токены и строит `HexDocument`.

Для этого парсер должен выдавать поток событий (Events), а `HexDocument` будет их потребителем (Consumer).
