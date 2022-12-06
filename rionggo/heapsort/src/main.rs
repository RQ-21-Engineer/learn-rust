use std::fs;
use std::io;

fn heapsort(arr: &mut [i32]) {
    let len = arr.len();

    // Build max heap
    for i in (0..len / 2).rev() {
        heapify(arr, i, len);
    }

    // Heap sort
    for i in (1..len).rev() {
        arr.swap(0, i);
        heapify(arr, 0, i);
    }
}

fn heapify(arr: &mut [i32], i: usize, len: usize) {
    let mut largest = i;
    let left = 2 * i + 1;
    let right = 2 * i + 2;

    if left < len && arr[left] > arr[largest] {
        largest = left;
    }
    if right < len && arr[right] > arr[largest] {
        largest = right;
    }

    if largest != i {
        arr.swap(i, largest);
        heapify(arr, largest, len);
    }
}

fn main() {
    let mut arr: Vec<i32> = Vec::new();

    // Prompt user for input method
    println!("Masukkan 1 untuk memasukkan data secara manual atau 2 untuk membaca dari file: ");
    let mut input_method = String::new();
    io::stdin()
        .read_line(&mut input_method)
        .expect("Gagal membaca baris");

    // Read input data
    if input_method.trim() == "1" {
        // Prompt user for input
        println!("Masukkan daftar angka, dipisahkan dengan spasi:");
        let mut user_input = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Gagal membaca baris");

        // Convert input string to array of integers
        arr = user_input
            .split_whitespace()
            .map(|x| x.parse().expect("Failed to parse input"))
            .collect();
    } else if input_method.trim() == "2" {
        // Prompt user for file path
        println!("Masukkan path file: ");
        let mut file_path = String::new();
        io::stdin()
            .read_line(&mut file_path)
            .expect("Gagal membaca baris");

        // Read file contents to string
        let file_contents = fs::read_to_string(file_path.trim())
            .expect("Gagal membaca baris");

        // Convert file contents to array of integers
        arr = file_contents
            .split_whitespace()
            .map(|x| x.parse().expect("Gagal mengurai masukan"))
            .collect();
    } else {
        println!("Metode masukan tidak valid");
        return;
    }

    // Sort array using heapsort
    heapsort(&mut arr);

    // Print sorted array
    println!("Array yang diurutkan: {:?}", arr);
}