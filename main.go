package main

import (
	"encoding/json"
	"errors"
	"fmt"
	"log"
	"os"
	"path/filepath"
)

type Couple struct {
   RemoteName string `json:"remote"`
   RemotePath string `json:"remote_path"`
   LocalPath  string `json:"local_path"`
}

const DB_PATH = "./db.json"

func main() {
   // create db if it doesn't exist
   if _, err := os.Stat(DB_PATH); errors.Is(err, os.ErrNotExist) {
      os.Create(DB_PATH)
      if abs, err := filepath.Abs(DB_PATH); err == nil {
         fmt.Printf("creating db file: %s\n", abs)
      } else {
         log.Fatal(err)
      }
   }
   // db_file, err := os.OpenFile(DB_PATH, os.O_CREATE | os.O_RDWR, os.ModeAppend)
   // if err != nil {
   //    log.Fatal(err)
   // }

   // var db map[string]interface{}
   var db []Couple
   if data, err := os.ReadFile(DB_PATH); err == nil {
      json.Unmarshal(data, &db)
   } else {
      log.Fatal(err)
   }
   fmt.Printf("I am: %s\n", os.Args[0])
   fmt.Printf("db: %s\n", db)
   for i,cup := range db {
      fmt.Printf("couple: %s at index %d\n", cup, i)
   }
}
