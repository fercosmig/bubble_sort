fn inverte_elementos(lista: &mut [i32; 7], i: usize, j: usize)
{
    let aux: i32 =  lista[i];
    lista[i] = lista[j];
    lista[j] = aux;
}

fn main()
{
    let mut array: [i32; 7] = [10, 23, 4, 5, 66, 7, -3];
    println!("{:?}", array);

    for i in 0..array.len()
    {
        for j in ( (i + 1)..array.len() ).rev()
        {
            println!("i: {}, j: {}", i, j);

            if array[j-1] > array[j]
            {
                inverte_elementos(&mut array, j - 1, j);
            }
        }
    }
    println!("{:?}", array);
}