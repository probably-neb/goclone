package main

import (
	"encoding/json"
	"errors"
	"fmt"
	"os/exec"

	// "io/fs"
	"log"
	"os"
	"path/filepath"
)

const DB_PATH = "./db.json"
const DB_SIZEI = 128

type DB = map[string]string

// var PathNotInDbError = errors.New("Local path not found in Database")

func load_db() DB {
	// TODO: return err instead of log.Fatal
	var db DB
	// create db if it doesn't exist
	if _, err := os.Stat(DB_PATH); errors.Is(err, os.ErrNotExist) {
		os.Create(DB_PATH)
		if abs, err := filepath.Abs(DB_PATH); err == nil {
			fmt.Printf("creating db file: %s\n", abs)
			db := make(DB, DB_SIZEI)
			return db
		} else {
			log.Fatal(err)
		}
	} else {
		if data, err := os.ReadFile(DB_PATH); err == nil {
			if json_err := json.Unmarshal(data, &db); json_err != nil {
				log.Fatal(json_err)
			}
		} else {
			log.Fatal(err)
		}
	}
	// fmt.Printf("I am: %s\n", os.Args[0])
	// fmt.Printf("db: %s\n", db)
	return db
}

func write_db(db DB) error {
	data, err := json.MarshalIndent(db, "", "  ")
	fmt.Printf("db json: %s\n", data)
	if err == nil {
		err = os.WriteFile(DB_PATH, data, 0666)
	}
	return err
}

func copy(db DB, local_path string, from bool) error {
	if remote_path, found := db[local_path]; !found {
		return fmt.Errorf("local path: %s not in Databse", local_path)
	} else {
		source := local_path
		dest := remote_path
		if from {
			source = remote_path
			dest = local_path
		}
		fmt.Printf("copying: %s -> %s\n", source, dest)
		cmd := exec.Command("rclone", "copy", source, dest)
		fmt.Printf("%s\n", cmd)
		output, err := cmd.Output()
		fmt.Println(string(output))
		if err != nil {
			return err
		}
	}
	return nil
}

func usage() {
	println("USAGE: goclone add local/path remote:path")
}

func main() {
	argc := len(os.Args)
	fmt.Printf("argc: %d\n", argc)
	if argc < 1 {
		return
	}

	db := load_db()
	switch os.Args[1] {
	case "add":
		if argc != 4 {
			usage()
		}
		// TODO: check input
		local_path, err := filepath.Abs(os.Args[2])
		if err != nil {
			log.Fatal(err)
		}
		db[local_path] = os.Args[3]
		fmt.Printf("db: %s\n", db)
	case "copy":
		if argc > 4 {
			usage()
		}
		from := false
		local_path_idx := 2
		if argc == 4 {
			switch os.Args[2] {
			case "--from", "-f":
				local_path_idx = 3
				from = true
			default:
				switch os.Args[3] {
				case "--from", "-f":
					from = true
				}
			}
		}
		local_path, err := filepath.Abs(os.Args[local_path_idx])

		if err != nil {
			log.Fatal(err)
		}
		err = copy(db, local_path, from)
		if err != nil {
			log.Fatal(err)
		}
   case "list":
      for k,v := range db {
         fmt.Printf("%s -> %s\n", k,v)
      }
	}
	err := write_db(db)
	if err != nil {
		log.Fatal(err)
	}
}
