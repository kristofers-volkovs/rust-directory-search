/*
 * To change this license header, choose License Headers in Project Properties.
 * To change this template file, choose Tools | Templates
 * and open the template in the editor.
 */
package pkg2pc.m03ConcurrencyAPI;

/**
 *
 * @author Vairis
 */
import java.io.File;
import java.nio.file.Files;
import java.util.ArrayList;
import java.util.List;
import java.util.concurrent.ForkJoinPool;
import java.util.concurrent.RecursiveTask;

public class Activity extends RecursiveTask <List<String>> {

   //full path of the folder this task is going to process.
   private final String path;
   //name of the extension of the files this task is going to look for.
   private final String extension;

   //Implement the constructor of the class to initialize its attributes
   public Activity(String path, String extension)
   {
      this.path = path;
      this.extension = extension;
   }

   //Implement the compute() method. As you parameterized the RecursiveTask class with the List<String> type, 
   //this method has to return an object of that type.
   @Override
   protected List<String> compute()
   {
      //List to store the names of the files found in the folder.
      List<String> list = new ArrayList<String>();
      //Activity tasks to store the subtasks that are going to process the subfolders stored in the folder
      List<Activity> tasks = new ArrayList<Activity>();
      //Get the content of the folder.
      File file = new File(path);
      File content[] = file.listFiles();

		//TODO - fill in

      //Return the list of strings
      return list;
   }

   //This method compares if the name of a file passed as a parameter ends with the extension you are looking for.
   private boolean checkFileExtension(String name)
   {
      return name.endsWith(extension);
   }

   //Write main methods executing task or tasks. Give a live feedback. Show results.
   public static void main(String args[]){
      final ForkJoinPool pool = new ForkJoinPool(4);
      final Activity finder = new Activity("E:\\vairis\\files", ".txt");
      List<String> endList = pool.invoke(finder);
      for(String s:endList)
      {
         System.out.println(s);
      }
      // System.out.println(pool.invoke(finder));
   }
}
