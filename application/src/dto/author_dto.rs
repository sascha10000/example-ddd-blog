use domain::entities::Author;

pub struct AuthorDto {
    id: u64,
    name: String, 
    surname: String
}

impl From<Author> for AuthorDto {
    fn from(value: Author) -> Self {
        AuthorDto { id: value.id, name: value.name, surname: value.surname }
    }
}
