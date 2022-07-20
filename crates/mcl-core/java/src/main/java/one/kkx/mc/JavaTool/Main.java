package one.kkx.mc.JavaTool;

import java.lang.reflect.Field;
import java.lang.reflect.Method;
import java.lang.Class;

public final class Main {
	public static void main(String[] args) {
		switch (args[0]) {
			case "props":
				String[] props = args[1].split(",");
				for (String prop : props) {
					System.out.println(prop + "=" + System.getProperty(prop));
				}
				break;
			case "legacyLaunch":
				try {
					System.out.println("Loading class " + args[1]);
					ClassLoader classLoader = ClassLoader.getSystemClassLoader();
					Class<?> clazz = classLoader.loadClass(args[1]);
					System.out.println(clazz.getName());
					for (Field field : clazz.getDeclaredFields()) {
						System.out.println("field");
						System.out.println(field.getName());
					}
				}
				catch(Exception e) {
					System.out.println(e.toString());
				}
				break;
		}
	}
}
